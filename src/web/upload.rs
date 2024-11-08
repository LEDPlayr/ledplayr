use std::{io::Write, path::Path, sync::Arc};

use axum::{
    extract::{self},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_typed_multipart::TypedMultipart;
use parking_lot::Mutex;

use crate::{db, models::*, state::State, storage, web::error::APIError};

/// Upload a file
///
/// Accepts fseq sequences or media files such as
/// images and videos. The uploaded file is automatically
/// sorted into the relevant upload directory so a call to
/// `moveFile` isn't required and will be ignore.
#[utoipa::path(
    post,
    path = "/api/upload",
    request_body(content_type = "multipart/form-data", content = FileUpload),
    responses(
        (status = 200, description = "File uploaded successfully", body = Status),
        (status = 400, description = "Unrecognized file type", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Upload"
)]
pub async fn file_upload(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    TypedMultipart(FileUpload { myfile }): TypedMultipart<FileUpload>,
) -> Response {
    for f in myfile.into_iter() {
        if let Some(filename) = f.metadata.file_name {
            let dir = match storage::get_dir(&filename) {
                Some(d) => d,
                None => {
                    return APIError::BadRequest("Unrecognized file type".into()).into_response();
                }
            };

            let mut state = state.lock();
            let path = Path::new(&state.cfg.storage)
                .join(dir.to_string())
                .join(&filename);
            tracing::info!("Processing upload: {:?}", path);

            let mut file = match std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)
            {
                Ok(file) => file,
                Err(e) => {
                    tracing::error!("Error opening file: {e}");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Status {
                            status: "error".into(),
                            error: Some(e.to_string()),
                        }),
                    )
                        .into_response();
                }
            };

            if let Err(e) = file.write_all(&f.contents) {
                tracing::error!("Error writing file: {e}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Status {
                        status: "error".into(),
                        error: Some(e.to_string()),
                    }),
                )
                    .into_response();
            }

            // Check sequences are valid and add to database
            if let storage::StorageType::Sequences = dir {
                if let Ok(Some(meta)) = storage::read_sequence_meta(&state.cfg, &filename) {
                    if let Err(e) = db::new_sequence(&mut state.db_conn, meta) {
                        tracing::error!("Error adding sequence to database: {e}");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(Status {
                                status: "error".into(),
                                error: Some(e.to_string()),
                            }),
                        )
                            .into_response();
                    }
                }
            }
        }
    }

    (
        StatusCode::OK,
        Json(Status {
            status: "ok".into(),
            error: None,
        }),
    )
        .into_response()
}
