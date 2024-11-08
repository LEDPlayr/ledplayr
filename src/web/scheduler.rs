use std::sync::Arc;

use axum::{
    extract,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{models::*, state::State, web::error::APIError};

/// Get the scheduler status
#[utoipa::path(
    get,
    path = "/api/scheduler",
    responses(
        (status = 200, description = "Status of the scheduler", body = SchedulerStatus),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scheduler"
)]
pub async fn get_scheduler_status(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
) -> Response {
    let state = state.lock();

    Json(SchedulerStatus {
        status: state.player_status.clone(),
    })
    .into_response()
}

/// Start the scheduler
#[utoipa::path(
    get,
    path = "/api/scheduler/start",
    responses(
        (status = 200, description = "Scheduler started ok", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scheduler"
)]
pub async fn start_scheduler(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let ctrl;
    {
        let state = state.lock();
        ctrl = state.player_ctrl.clone();
    }

    if let Err(e) = ctrl.send(PlayerState::Start).await {
        tracing::error!("Could not start scheduler: {e}");
        return APIError::UnexpectedError(e.into()).into_response();
    }

    APIError::Ok.into_response()
}

/// Stop the scheduler
#[utoipa::path(
    get,
    path = "/api/scheduler/stop",
    responses(
        (status = 200, description = "Scheduler stopped ok", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scheduler"
)]
pub async fn stop_scheduler(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let ctrl;
    {
        let state = state.lock();
        ctrl = state.player_ctrl.clone();
    }

    if let Err(e) = ctrl.send(PlayerState::Stop).await {
        tracing::error!("Could not stop scheduler: {e}");
        return APIError::UnexpectedError(e.into()).into_response();
    }

    APIError::Ok.into_response()
}
