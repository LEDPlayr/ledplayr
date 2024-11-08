use std::sync::Arc;

use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{db, models::*, state::State, web::error::APIError};

/// List schedules
///
/// List the name of all schedules
#[utoipa::path(
    get,
    path = "/api/schedules",
    responses(
        (status = 200, description = "List of available schedules", body = [Schedule]),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Schedules"
)]
pub async fn list_schedules(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock();

    match db::get_schedules(&mut state.db_conn) {
        Ok(schedules) => {
            let schedules: Vec<Schedule> = schedules
                .into_iter()
                .filter_map(|s| s.try_into().ok())
                .collect();
            Json(schedules).into_response()
        }
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Get a schedule
///
/// Read back a schedule
#[utoipa::path(
    get,
    path = "/api/schedule/{schedule}",
    params(
        ("schedule" = String, Path, description = "The name of the schedule")
    ),
    responses(
        (status = 200, description = "The requested schedule", body = Schedule),
        (status = 404, description = "The schedule wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Schedules"
)]
pub async fn get_schedule(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(schedule): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    match db::get_schedule(&mut state.db_conn, schedule) {
        Ok(Some(s)) => match Schedule::try_from(s) {
            Ok(schedule) => (StatusCode::OK, Json(schedule)).into_response(),
            Err(e) => APIError::BadRequest(e).into_response(),
        },
        Ok(None) => APIError::NotFound("Schedule".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// New schedule
///
/// Create a new schedule
#[utoipa::path(
    post,
    path = "/api/schedule",
    request_body(content = Schedule),
    responses(
        (status = 201, description = "The schedule was created", body = Status),
        (status = 422, description = "Incomplete schedule supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Schedules"
)]
pub async fn new_schedule(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(schedule): Json<Schedule>,
) -> Response {
    create_or_update_schedule(state, schedule).await
}

/// Update a schedule
///
/// Create or update the given schedule
#[utoipa::path(
    put,
    path = "/api/schedule/{schedule}",
    params(
        ("schedule" = String, Path, description = "The name of the schedule")
    ),
    request_body(content = Schedule),
    responses(
        (status = 200, description = "The requested schedule was updated", body = Status),
        (status = 422, description = "Incomplete schedule supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Schedules"
)]
pub async fn update_schedule(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(schedule_name): extract::Path<String>,
    Json(mut schedule): Json<Schedule>,
) -> Response {
    schedule.name = schedule_name;
    create_or_update_schedule(state, schedule).await
}

/// Delete a schedule
///
/// Delete the given schedule
#[utoipa::path(
    delete,
    path = "/api/schedule/{schedule}",
    params(
        ("schedule" = String, Path, description = "The name of the schedule")
    ),
    responses(
        (status = 200, description = "The requested schedule was deleted", body = Status),
        (status = 404, description = "The schedule wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Schedules"
)]
pub async fn del_schedule(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(schedule): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    match db::del_schedule(&mut state.db_conn, schedule) {
        Ok(Some(_)) => APIError::Ok.into_response(),
        Ok(None) => APIError::NotFound("Schedule".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

async fn create_or_update_schedule(state: Arc<Mutex<State>>, schedule: Schedule) -> Response {
    if schedule.name.is_empty() {
        return APIError::BadRequest("Name cannot be empty".into()).into_response();
    }

    let mut state = state.lock();

    let schedule = match db::models::NewSchedule::try_from(schedule) {
        Ok(s) => s,
        Err(e) => return APIError::BadRequest(e).into_response(),
    };

    match db::new_schedule(&mut state.db_conn, schedule) {
        Ok(_) => APIError::Ok.into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}
