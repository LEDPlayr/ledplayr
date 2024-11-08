use std::{env, sync::Arc};

use anyhow::{anyhow, Context, Result};

use dotenvy::dotenv;
use ledplayr::{
    built_info,
    config::Config,
    db,
    error::AppError,
    fpp,
    models::{PlayerState, PlayerStatus},
    player,
    state::State,
    storage,
    web::router,
};
use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt::writer::MakeWriterExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cfg_file = env::var("CONFIG").map_err(|_| anyhow!("CONFIG envvar is required"))?;
    let cfg =
        std::fs::read_to_string(cfg_file).map_err(|e| AppError::ConfigError(e.to_string()))?;
    let cfg: Config = toml::from_str(&cfg).map_err(|e| AppError::ConfigError(e.to_string()))?;

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    if let Some(log) = &cfg.log {
        let period = match log.period {
            Some(ledplayr::config::LogPeriod::Minute) => Rotation::MINUTELY,
            Some(ledplayr::config::LogPeriod::Hour) => Rotation::HOURLY,
            Some(ledplayr::config::LogPeriod::Day) => Rotation::DAILY,
            Some(ledplayr::config::LogPeriod::Never) => Rotation::NEVER,
            None => Rotation::NEVER,
        };

        let mut logfile = RollingFileAppender::builder().rotation(period);

        if let Some(prefix) = &log.prefix {
            logfile = logfile.filename_prefix(prefix);
        }

        if let Some(max_files) = &log.max_files {
            logfile = logfile.max_log_files(*max_files);
        }

        let logfile = logfile
            .build(&log.directory)
            .context("failed to initialize rolling log")?;

        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(std::io::stdout.and(logfile))
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(std::io::stdout)
            .init();
    }

    let built_time = chrono::DateTime::parse_from_rfc2822(built_info::BUILT_TIME_UTC)
        .unwrap()
        .with_timezone(&chrono::offset::Local);
    let version = built_info::GIT_VERSION.unwrap_or("unknown");

    tracing::info!(
        "Starting {} {} built {}",
        built_info::PKG_NAME,
        version,
        built_time
    );

    tracing::info!("Setting up database");
    let mut db_conn = db::get(&cfg)?;
    db::run_migrations(&mut db_conn)?;

    tracing::info!("Configuring storage");
    storage::init(&cfg)?;

    let multicast_enabled = cfg.multicast.unwrap_or(true);
    let tracker = TaskTracker::new();
    let cancel = CancellationToken::new();
    let (player_ctrl, player_state) = mpsc::channel(1);

    let mut auto_start = true;
    if let Some(scheduler_config) = &cfg.scheduler {
        auto_start = scheduler_config.auto_start.unwrap_or(true);
    }
    if auto_start {
        if let Err(e) = player_ctrl.send(PlayerState::Start).await {
            tracing::error!("Could not start scheduler: {e}");
        }
    }

    let state = Arc::new(Mutex::new(State {
        cfg,
        db_conn,
        player_ctrl,
        player_status: PlayerStatus::Stop,
    }));

    if multicast_enabled {
        tracker.spawn(fpp::listen(cancel.clone()));
    }
    tracker.spawn(router::run_server(state.clone(), cancel.clone()));
    tracker.spawn(player::start_scheduler(
        state.clone(),
        cancel.clone(),
        player_state,
    ));

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    tracker.close();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Quit, starting shutdown");
        },
        _ = terminate => {
            tracing::info!("Terminated, starting shutdown");
        },
        _ = cancel.cancelled() => {
            tracing::info!("Thread died, starting shutdown");
        },
    }

    cancel.cancel();
    tracker.wait().await;

    Ok(())
}
