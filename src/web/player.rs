use std::sync::Arc;

use axum::{
    extract,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{models::*, patterns::TestSpec, state::State, web::error::APIError};

/// Get the player status
#[utoipa::path(
    get,
    path = "/api/player",
    responses(
        (status = 200, description = "Status of the scheduler", body = PlayerStatus),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Player"
)]
pub async fn get_status(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let state = state.lock();

    Json(state.player_status.clone()).into_response()
}

/// Start the player scheduling
#[utoipa::path(
    get,
    path = "/api/player/schedule",
    responses(
        (status = 200, description = "Player started ok", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Player"
)]
pub async fn start_scheduler(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let ctrl;
    {
        let state = state.lock();
        ctrl = state.player_ctrl.clone();
    }

    if let Err(e) = ctrl.send(PlayerState::Schedule).await {
        tracing::error!("Could not start player: {e}");
        return APIError::UnexpectedError(e.into()).into_response();
    }

    APIError::Ok.into_response()
}

/// Stop the player
#[utoipa::path(
    get,
    path = "/api/player/stop",
    responses(
        (status = 200, description = "Player stopped ok", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Player"
)]
pub async fn stop(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let ctrl;
    {
        let state = state.lock();
        ctrl = state.player_ctrl.clone();
    }

    if let Err(e) = ctrl.send(PlayerState::Stop).await {
        tracing::error!("Could not stop player: {e}");
        return APIError::UnexpectedError(e.into()).into_response();
    }

    APIError::Ok.into_response()
}

/// Run LED test patterns
#[utoipa::path(
    post,
    path = "/api/player/test",
    request_body(content = TestSpec),
    responses(
        (status = 200, description = "Tests started ok", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Player"
)]
pub async fn run_test(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(test_spec): Json<TestSpec>,
) -> Response {
    let ctrl;
    {
        let state = state.lock();
        ctrl = state.player_ctrl.clone();
    }

    if let Err(e) = ctrl.send(PlayerState::Test(test_spec)).await {
        tracing::error!("Could not start tests: {e}");
        return APIError::UnexpectedError(e.into()).into_response();
    }

    APIError::Ok.into_response()
}
