use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::Status;

#[derive(Debug)]
pub enum APIError {
    Ok,
    NotFound(String),
    BadRequest(String),
    UnexpectedError(anyhow::Error),
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        match self {
            APIError::Ok => Json(Status {
                status: "ok".into(),
                error: None,
            })
            .into_response(),
            APIError::NotFound(s) => (
                StatusCode::NOT_FOUND,
                Json(Status {
                    status: "error".into(),
                    error: Some(format!("{s} not found")),
                }),
            )
                .into_response(),
            APIError::BadRequest(e) => (
                StatusCode::BAD_REQUEST,
                Json(Status {
                    status: "error".into(),
                    error: Some(e),
                }),
            )
                .into_response(),
            APIError::UnexpectedError(e) => {
                tracing::error!("An unexpected error occured: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Status {
                        status: "error".into(),
                        error: Some(e.to_string()),
                    }),
                )
                    .into_response()
            }
        }
    }
}
