use std::sync::Arc;

use axum::{
    extract,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{db, models::*, state::State, storage, web::error::APIError};

/// List all sequences
///
/// List all sequence files
#[utoipa::path(
    get,
    path = "/api/sequences",
    responses(
        (status = 200, description = "The requested sequence", body = Vec<String>),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Sequences"
)]
pub async fn list_sequences(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock();

    match db::get_sequences(&mut state.db_conn) {
        Ok(seqs) => Json(seqs.into_iter().map(|s| s.name).collect::<Vec<String>>()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Get a sequence
///
/// Download a sequence file
#[utoipa::path(
    get,
    path = "/api/sequence/{filename}",
    params(
        ("filename" = String, Path, description = "The sequence to download")
    ),
    responses(
        (status = 200, description = "The requested sequence", body = Vec<u8>),
        (status = 404, description = "The sequence wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Sequences"
)]
pub async fn get_sequence(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(filename): extract::Path<String>,
) -> Response {
    let state = state.lock();

    match storage::read_file(&state.cfg, &filename, storage::StorageType::Sequences) {
        Ok(Some(data)) => data.into_response(),
        Ok(None) => APIError::NotFound("Sequence".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Delete a sequence
///
/// Remove a sequence file
#[utoipa::path(
    delete,
    path = "/api/sequence/{filename}",
    params(
        ("filename" = String, Path, description = "The sequence to download")
    ),
    responses(
        (status = 200, description = "The sequence was removed", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Sequences"
)]
pub async fn del_sequence(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(filename): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    if let Err(e) = storage::del_file(&state.cfg, &filename, storage::StorageType::Sequences) {
        return APIError::UnexpectedError(e).into_response();
    }

    match db::del_sequence(&mut state.db_conn, filename) {
        Ok(Some(_)) => APIError::Ok.into_response(),
        Ok(None) => APIError::NotFound("Sequence".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Get a sequence's metadata
///
/// Get the metadata belonging to a sequence
#[utoipa::path(
    get,
    path = "/api/sequence/{filename}/meta",
    params(
        ("filename" = String, Path, description = "The sequence to download")
    ),
    responses(
        (status = 200, description = "The requested sequence", body = SequenceMeta),
        (status = 404, description = "The sequence wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Sequences"
)]
pub async fn get_sequence_meta(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(filename): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    match db::get_sequence(&mut state.db_conn, filename) {
        Ok(Some((seq, vars))) => Json(SequenceMeta {
            name: seq.name,
            id: seq.timestamp,
            step_time: seq.step_time as u8,
            num_frames: seq.frames as u32,
            channel_count: seq.channels as u32,
            variables: vars.into_iter().map(|v| (v.name, v.value)).collect(),
        })
        .into_response(),
        Ok(None) => APIError::NotFound("Sequence".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}
