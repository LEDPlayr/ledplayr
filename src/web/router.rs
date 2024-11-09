use std::{net::Ipv4Addr, sync::Arc};

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use parking_lot::Mutex;
use tokio_util::sync::CancellationToken;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipauto::utoipauto;

use crate::{
    state::State,
    web::{
        files, fpp, logs, meshes, playlists, scenes, scheduler, schedules, sequences, testing,
        upload,
    },
};

#[utoipauto]
#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

pub async fn run_server(state: Arc<Mutex<State>>, cancel: CancellationToken) {
    let mut ip = Ipv4Addr::new(0, 0, 0, 0);
    let mut port = 3000;

    if let Some(web) = &state.lock().cfg.web {
        if let Some(bind) = web.bind {
            ip = bind;
        }
        if let Some(p) = web.port {
            port = p;
        }
    }

    let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
        .await
        .context("Web listener failed")
        .unwrap();

    tracing::info!(
        "Listening on {}",
        listener
            .local_addr()
            .context("Failed to get local address")
            .unwrap()
    );

    let app = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
        .route("/jqupload.php", post(upload::file_upload))
        .route("/api/upload", post(upload::file_upload))
        .route("/fppxml.php", get(fpp::fpp_command))
        .route("/api/sequences", get(sequences::list_sequences))
        .route(
            "/api/sequence/:filename",
            get(sequences::get_sequence).delete(sequences::del_sequence),
        )
        .route(
            "/api/sequence/:filename/meta",
            get(sequences::get_sequence_meta),
        )
        .route("/api/playlists", get(playlists::list_playlists))
        .route(
            "/api/playlists/numbered",
            get(playlists::list_playlists_numbered),
        )
        .route("/api/playlist", post(playlists::new_playlist))
        .route(
            "/api/playlist/:playlist",
            get(playlists::get_playlist)
                .post(playlists::new_playlist)
                .put(playlists::update_playlist)
                .delete(playlists::del_playlist),
        )
        .route("/api/schedules", get(schedules::list_schedules))
        .route("/api/schedule", post(schedules::new_schedule))
        .route(
            "/api/schedule/:schedule",
            get(schedules::get_schedule)
                .put(schedules::update_schedule)
                .delete(schedules::del_schedule),
        )
        .route("/api/system/info", get(fpp::system_info))
        .route(
            "/api/models",
            get(fpp::list_models).post(fpp::upload_models),
        )
        .route(
            "/api/configfile/virtualdisplaymap",
            get(fpp::get_display).post(fpp::upload_display),
        )
        .route(
            "/api/channel/output/universeOutputs",
            get(fpp::get_outputs).post(fpp::upload_outputs),
        )
        .route("/api/scheduler", get(scheduler::get_scheduler_status))
        .route("/api/scheduler/start", get(scheduler::start_scheduler))
        .route("/api/scheduler/stop", get(scheduler::stop_scheduler))
        .route("/api/test/run", post(testing::run_test))
        .route("/api/test/sequence", post(testing::get_test_sequence))
        .route("/api/logs", get(logs::list_logs))
        .route("/api/log/:name", get(logs::get_log))
        .route("/api/meshes", get(meshes::list_meshes))
        .route("/api/mesh", post(meshes::new_mesh))
        .route(
            "/api/mesh/:mesh",
            get(meshes::download_mesh)
                .put(meshes::update_mesh)
                .delete(meshes::del_mesh),
        )
        .route("/api/scenes", get(scenes::list_scenes))
        .route("/api/scene", post(scenes::new_scene))
        .route(
            "/api/scenes/:scene",
            get(scenes::get_scene)
                .put(scenes::update_scene)
                .delete(scenes::del_scene),
        )
        .fallback(files::static_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
                .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG)),
        )
        .with_state(state.clone());

    match axum::serve(listener, app)
        .with_graceful_shutdown(async move { cancel.cancelled().await })
        .await
    {
        Ok(_) => {}
        Err(e) => tracing::error!("Axum exited with an error: {}", e),
    }
}
