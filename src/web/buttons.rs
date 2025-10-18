use std::sync::Arc;

use axum::{
    extract,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{
    button,
    db::{
        self,
        models::{Button, NewButton},
    },
    models::Status,
    state::State,
};

use super::error::APIError;

/// List all buttons
///
/// List all remote buttons
#[utoipa::path(
    get,
    path = "/api/buttons",
    responses(
        (status = 200, description = "List of buttons", body = Vec<Button>),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Buttons"
)]
pub async fn list_buttons(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock();

    match db::get_buttons(&mut state.db_conn) {
        Ok(m) => Json(m).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Get a button
///
/// Get a single button
#[utoipa::path(
    get,
    path = "/api/button/{button}",
    params(
        ("button" = i32, Path, description = "The ID of the button")
    ),
    responses(
        (status = 200, description = "The requested button", body = Button),
        (status = 404, description = "Unknown button", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Buttons"
)]
pub async fn get_button(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(button_id): extract::Path<i32>,
) -> Response {
    let mut state = state.lock();

    match db::get_button(&mut state.db_conn, button_id) {
        Ok(Some(d)) => Json(d).into_response(),
        Ok(None) => APIError::NotFound("Button not found".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// New button
///
/// Create a new button
#[utoipa::path(
    post,
    path = "/api/button",
    request_body(content = NewButton),
    responses(
        (status = 201, description = "The button was created", body = Status),
        (status = 422, description = "Incomplete button supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Buttons"
)]
pub async fn new_button(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(button): Json<NewButton>,
) -> Response {
    let mut state = state.lock();

    match db::new_button(&mut state.db_conn, button) {
        Ok(_) => APIError::Ok.into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Update a button
///
/// Update the given button
#[utoipa::path(
    put,
    path = "/api/button/{button}",
    params(
        ("button" = i32, Path, description = "The ID of the button")
    ),
    request_body(content = NewButton),
    responses(
        (status = 200, description = "The requested button was updated", body = Status),
        (status = 422, description = "Incomplete button supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Buttons"
)]
pub async fn update_button(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(button_id): extract::Path<i32>,
    Json(btn): Json<NewButton>,
) -> Response {
    button::update_button(button_id, btn, state).await;
    APIError::Ok.into_response()
}

/// Delete a button
///
/// Delete the given button
#[utoipa::path(
    delete,
    path = "/api/button/{button}",
    params(
        ("button" = i32, Path, description = "The ID of the button")
    ),
    responses(
        (status = 200, description = "The requested button was deleted", body = Status),
        (status = 404, description = "The button wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Buttons"
)]
pub async fn del_button(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(button_id): extract::Path<i32>,
) -> Response {
    let mut state = state.lock();

    match db::del_button(&mut state.db_conn, button_id) {
        Ok(Some(_)) => APIError::Ok.into_response(),
        Ok(None) => APIError::NotFound("Scene".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}
