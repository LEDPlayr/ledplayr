use std::{
    env,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Result};

use dotenvy::dotenv;
use ledplayr::{
    built_info, config::Config, db, error::AppError, fpp, models::PlayerState, player,
    state::State, storage, web,
};
use tokio::sync::mpsc;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let cfg_file = env::var("CONFIG").map_err(|_| anyhow!("CONFIG envvar is required"))?;

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

    tracing::info!("Opening config file: {cfg_file}");
    let cfg =
        std::fs::read_to_string(cfg_file).map_err(|e| AppError::ConfigError(e.to_string()))?;
    let cfg: Config = toml::from_str(&cfg).map_err(|e| AppError::ConfigError(e.to_string()))?;

    tracing::info!("Setting up database");
    let mut db_conn = db::get(&cfg)?;
    db::run_migrations(&mut db_conn)?;

    tracing::info!("Configuring storage");
    storage::init(&cfg)?;

    let multicast_enabled = cfg.multicast.unwrap_or(true);
    let tracker = TaskTracker::new();
    let cancel = CancellationToken::new();
    let (player_ctrl, player_state) = mpsc::channel(1);

    if let Err(e) = player_ctrl.send(PlayerState::Start).await {
        tracing::error!("Could not start scheduler: {e}");
    }

    let state = Arc::new(Mutex::new(State {
        cfg,
        db_conn,
        player_ctrl,
        player_state: PlayerState::Stop,
    }));

    if multicast_enabled {
        tracker.spawn(fpp::listen(cancel.clone()));
    }
    tracker.spawn(web::run_server(state.clone(), cancel.clone()));
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