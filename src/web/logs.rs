use std::{io::Read, sync::Arc};

use anyhow::anyhow;
use axum::{
    extract::{self},
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{state::State, web::error::APIError};

/// Get log filenames
#[utoipa::path(
    get,
    path = "/api/logs",
    responses(
        (status = 200, description = "List of log filenames", body = Vec<String>),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Logs"
)]
pub async fn list_logs(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let state = state.lock();

    // Based on
    // https://docs.rs/tracing-appender/latest/src/tracing_appender/rolling.rs.html#571
    let files = match &state.cfg.log {
        Some(log) => std::fs::read_dir(&log.directory)
            .map(|dir| {
                dir.filter_map(|entry| {
                    let entry = entry.ok()?;
                    let metadata = entry.metadata().ok()?;

                    if !metadata.is_file() {
                        return None;
                    }

                    let filename = entry.file_name();
                    let filename = filename.to_str()?;
                    if let Some(prefix) = &log.prefix {
                        if !filename.starts_with(prefix) {
                            return None;
                        }
                    }

                    if let Some(period) = &log.period {
                        if log.prefix.is_none()
                            && time::Date::parse(filename, &period.date_format()).is_err()
                        {
                            return None;
                        }
                    }

                    Some(filename.to_string())
                })
                .collect::<Vec<_>>()
            })
            .map_err(|e| anyhow!(e)),
        None => Err(anyhow!("Logging not enabled")),
    };

    match files {
        Ok(mut files) => {
            files.sort();
            Json(files).into_response()
        }
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Get a specific log
#[utoipa::path(
    get,
    path = "/api/log/{name}",
    params(
        ("name" = String, Path, description = "The name of the log to display")
    ),
    responses(
        (status = 200, description = "List of log filenames", body = String),
        (status = 400, description = "Logging is disabled", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Logs"
)]
pub async fn get_log(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(name): extract::Path<String>,
) -> Response {
    let state = state.lock();

    match &state.cfg.log {
        Some(log) => {
            let path = std::path::Path::new(&log.directory).join(name);
            match std::fs::OpenOptions::new().read(true).open(path) {
                Ok(mut logfile) => {
                    let mut data = String::new();
                    match logfile.read_to_string(&mut data) {
                        Ok(_) => data.into_response(),
                        Err(e) => APIError::UnexpectedError(e.into()).into_response(),
                    }
                }
                Err(e) => APIError::UnexpectedError(e.into()).into_response(),
            }
        }
        None => APIError::BadRequest("File logging is disabled".into()).into_response(),
    }
}
