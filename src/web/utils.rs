use std::sync::Arc;

use axum::{
    http::header,
    response::{IntoResponse, Response},
};
use parking_lot::Mutex;

use crate::{
    state::State,
    storage::{self, StorageType},
    web::error::APIError,
};

pub async fn upload_other(state: Arc<Mutex<State>>, filename: String, data: Vec<u8>) -> Response {
    let state = state.lock();

    match storage::upload_file(&state.cfg, &filename, StorageType::Other, data) {
        Ok(_) => APIError::Ok.into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

pub async fn download_other(
    state: Arc<Mutex<State>>,
    filename: String,
    mimetype: String,
) -> Response {
    let state = state.lock();

    match storage::read_file(&state.cfg, &filename, StorageType::Other) {
        Ok(Some(d)) => ([(header::CONTENT_TYPE, mimetype)], d).into_response(),
        Ok(None) => APIError::NotFound(filename).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}
