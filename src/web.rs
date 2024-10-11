use std::{
    io::Write,
    net::Ipv4Addr,
    path::Path,
    sync::{Arc, Mutex},
};

use axum::{
    body::Bytes,
    extract,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_typed_multipart::TypedMultipart;
use chrono::{Datelike, Timelike};
use humanize_duration::prelude::DurationExt;
use rust_embed::Embed;
use rustix::path::Arg;
use systemstat::Platform;
use tokio_util::sync::CancellationToken;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipauto::utoipauto;

use crate::{built_info, db::models::NewSequencePlus, models::*, storage::StorageType};
use crate::{
    db::{self},
    state::State,
    storage,
};

#[utoipauto]
#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Embed)]
#[folder = "web/build"]
struct StaticAssets;

static INDEX_HTML: &str = "index.html";

pub async fn run_server(state: Arc<Mutex<State>>, cancel: CancellationToken) {
    let mut ip = Ipv4Addr::new(0, 0, 0, 0);
    let mut port = 3000;

    if let Some(web) = &state.lock().unwrap().cfg.web {
        if let Some(bind) = web.bind {
            ip = bind;
        }
        if let Some(p) = web.port {
            port = p;
        }
    }

    let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
        .await
        .unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
        .route("/jqupload.php", post(file_upload))
        .route("/api/upload", post(file_upload))
        .route("/fppxml.php", get(fpp_command))
        .route("/api/sequences", get(list_sequences))
        .route(
            "/api/sequence/:filename",
            get(get_sequence).delete(del_sequence),
        )
        .route("/api/sequence/:filename/meta", get(get_sequence_meta))
        .route("/api/playlists", get(list_playlists))
        .route("/api/playlists/numbered", get(list_playlists_numbered))
        .route("/api/playlist", post(new_playlist))
        .route(
            "/api/playlist/:playlist",
            get(get_playlist)
                .post(new_playlist)
                .put(update_playlist)
                .delete(del_playlist),
        )
        .route("/api/schedules", get(list_schedules))
        .route("/api/schedule", post(new_schedule))
        .route(
            "/api/schedule/:schedule",
            get(get_schedule).put(update_schedule).delete(del_schedule),
        )
        .route("/api/system/info", get(system_info))
        .route("/api/models", get(get_models).post(upload_models))
        .route(
            "/api/configfile/virtualdisplaymap",
            get(get_display).post(upload_display),
        )
        .route(
            "/api/channel/output/universeOutputs",
            get(get_outputs).post(upload_outputs),
        )
        .route("/api/scheduler", get(get_scheduler_status))
        .route("/api/scheduler/start", get(start_scheduler))
        .route("/api/scheduler/stop", get(stop_scheduler))
        .fallback(static_handler)
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

/// Handle static files from the rust_embed
///
/// Applies logic to serve the asset directory as a SPA
async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match StaticAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

/// Handle access to index.html
async fn index_html() -> Response {
    match StaticAssets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

/// Basic 404 page
async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

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
async fn file_upload(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    TypedMultipart(FileUpload { myfile }): TypedMultipart<FileUpload>,
) -> Response {
    for f in myfile.into_iter() {
        if let Some(filename) = f.metadata.file_name {
            let dir = match storage::get_dir(&filename) {
                Some(d) => d,
                None => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(Status {
                            status: "error".into(),
                            error: Some("Unrecognized file type".into()),
                        }),
                    )
                        .into_response();
                }
            };

            let mut state = state.lock().unwrap();
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

/// Run an FPP Command
///
/// This method isn't really implemented. The only command you
/// can issue is `moveFile` and all that really does is check
/// whether a file exists or not - it doesn't move it because
/// that's handled at upload time.
#[utoipa::path(
    get,
    path = "/fppxml.php",
    params(
        ("command" = String, Query, description = "The FFP command", example = "moveFile"),
        ("file" = String, Query, description = "The file to move", example = "file.fseq"),
    ),
    responses(
        (status = 200, description = "File exists", body = Status),
        (status = 400, description = "Unrecognized command", body = Status),
        (status = 404, description = "File doesn't exist", body = Status)
    ),
    tag = "FPP Compatibility"
)]
async fn fpp_command(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    query: extract::Query<CommandQuery>,
) -> Response {
    let state = state.lock().unwrap();

    match query.command.as_str() {
        "moveFile" => {
            let dir = match storage::get_dir(&query.file) {
                Some(d) => d,
                None => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(Status {
                            status: "error".into(),
                            error: Some("Unrecognized file type".into()),
                        }),
                    )
                        .into_response();
                }
            };

            let path = Path::new(&state.cfg.storage)
                .join(dir.to_string())
                .join(&query.file);
            if path.exists() {
                (
                    StatusCode::OK,
                    Json(Status {
                        status: "ok".into(),
                        error: None,
                    }),
                )
                    .into_response()
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(Status {
                        status: "error".into(),
                        error: Some("File doesn't exist".into()),
                    }),
                )
                    .into_response()
            }
        }
        _ => (
            StatusCode::BAD_REQUEST,
            Json(Status {
                status: "error".into(),
                error: Some("Unrecognized command".into()),
            }),
        )
            .into_response(),
    }
}

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
async fn list_sequences(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock().unwrap();

    match db::get_sequences(&mut state.db_conn) {
        Ok(seqs) => (
            StatusCode::OK,
            Json(seqs.into_iter().map(|s| s.name).collect::<Vec<String>>()),
        )
            .into_response(),
        Err(e) => {
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
async fn get_sequence(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(filename): extract::Path<String>,
) -> Response {
    let state = state.lock().unwrap();

    match storage::read_file(&state.cfg, &filename, storage::StorageType::Sequences) {
        Ok(Some(data)) => (StatusCode::OK, data).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(Status {
                status: "error".into(),
                error: Some("Sequence not found".into()),
            }),
        )
            .into_response(),
        Err(e) => {
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
async fn del_sequence(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(filename): extract::Path<String>,
) -> Response {
    let mut state = state.lock().unwrap();

    if let Err(e) = storage::del_file(&state.cfg, &filename, storage::StorageType::Sequences) {
        tracing::error!("{e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Status {
                status: "error".into(),
                error: Some(e.to_string()),
            }),
        )
            .into_response();
    }

    match db::del_sequence(&mut state.db_conn, filename) {
        Ok(_) => (
            StatusCode::OK,
            Json(Status {
                status: "ok".into(),
                error: None,
            }),
        )
            .into_response(),
        Err(e) => {
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
async fn get_sequence_meta(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(filename): extract::Path<String>,
) -> Response {
    let mut state = state.lock().unwrap();

    match db::get_sequence(&mut state.db_conn, filename) {
        Ok(Some((seq, vars))) => (
            StatusCode::OK,
            Json(SequenceMeta {
                name: seq.name,
                id: seq.timestamp,
                step_time: seq.step_time as u8,
                num_frames: seq.frames as u32,
                channel_count: seq.channels as u32,
                variables: vars.into_iter().map(|v| (v.name, v.value)).collect(),
            }),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(Status {
                status: "error".into(),
                error: Some("Sequence not found".into()),
            }),
        )
            .into_response(),
        Err(e) => {
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

/// List playlists
///
/// List the name of all playlists
#[utoipa::path(
    get,
    path = "/api/playlists",
    responses(
        (status = 200, description = "List of available playlists", body = [String]),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Playlists"
)]
async fn list_playlists(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock().unwrap();

    match db::get_playlists(&mut state.db_conn) {
        Ok(playlists) => {
            let ret: Vec<String> = playlists.iter().map(|p| p.name.clone()).collect();
            Json(ret).into_response()
        }
        Err(e) => {
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

/// List playlists with ID
///
/// List the playlists with their ID
#[utoipa::path(
    get,
    path = "/api/playlists/numbered",
    responses(
        (status = 200, description = "List of available playlists", body = [NumberedPlaylist]),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Playlists"
)]
async fn list_playlists_numbered(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
) -> Response {
    let mut state = state.lock().unwrap();

    match db::get_playlists(&mut state.db_conn) {
        Ok(playlists) => {
            let ret: Vec<NumberedPlaylist> = playlists
                .into_iter()
                .map(|p| NumberedPlaylist {
                    name: p.name,
                    id: p.id,
                })
                .collect();
            Json(ret).into_response()
        }
        Err(e) => {
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

/// Get a playlist
///
/// Read back a playlist
#[utoipa::path(
    get,
    path = "/api/playlist/{playlist}",
    params(
        ("playlist" = String, Path, description = "The name of the playlist")
    ),
    responses(
        (status = 200, description = "The requested playlist", body = Playlist),
        (status = 404, description = "The playlist wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Playlists"
)]
async fn get_playlist(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(playlist): extract::Path<String>,
) -> Response {
    let mut state = state.lock().unwrap();

    match db::get_playlist(&mut state.db_conn, playlist) {
        Ok(Some((p, s))) => {
            let mut info = PlaylistInfo::default();

            let sequences: Vec<PlaylistEntry> = s
                .into_iter()
                .map(|s| {
                    let duration = (s.sequence.step_time * s.sequence.frames) as f32 / 1000.0;

                    info.total_duration += duration;
                    info.total_items += 1;

                    PlaylistEntry {
                        duration: Some(duration),
                        sequence_name: s.sequence.name,
                        play_once: s.play_once,
                        enabled: s.enabled,
                        playlist_type: "sequence".into(),
                    }
                })
                .collect();

            (
                StatusCode::OK,
                Json(Playlist {
                    name: Some(p.name),
                    version: 3,
                    repeat: p.repeat,
                    loop_count: p.loop_count,
                    empty: sequences.is_empty(),
                    desc: p.description,
                    random: false,
                    lead_in: Vec::new(),
                    main_playlist: sequences,
                    lead_out: Vec::new(),
                    playlist_info: Some(info),
                }),
            )
                .into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(Status {
                status: "error".into(),
                error: Some("Playlist not found".into()),
            }),
        )
            .into_response(),
        Err(e) => {
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

/// New playlist
///
/// Create a new playlist
#[utoipa::path(
    post,
    path = "/api/playlist",
    request_body(content = Playlist),
    responses(
        (status = 201, description = "The playlist was created", body = Status),
        (status = 422, description = "Incomplete playlist supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Playlists"
)]
async fn new_playlist(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(playlist): Json<Playlist>,
) -> Response {
    create_or_update_playlist(state, playlist).await
}

/// Update a playlist
///
/// Create or update the given playlist
#[utoipa::path(
    put,
    path = "/api/playlist/{playlist}",
    params(
        ("playlist" = String, Path, description = "The name of the playlist")
    ),
    request_body(content = Playlist),
    responses(
        (status = 200, description = "The requested playlist", body = Status),
        (status = 422, description = "Incomplete playlist supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Playlists"
)]
async fn update_playlist(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(playlist_name): extract::Path<String>,
    Json(mut playlist): Json<Playlist>,
) -> Response {
    playlist.name = Some(playlist_name);
    create_or_update_playlist(state, playlist).await
}

/// Delete a playlist
///
/// Delete the given playlist
#[utoipa::path(
    delete,
    path = "/api/playlist/{playlist}",
    params(
        ("playlist" = String, Path, description = "The name of the playlist")
    ),
    responses(
        (status = 200, description = "The requested playlist was deleted", body = Status),
        (status = 404, description = "The playlist wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Playlists"
)]
async fn del_playlist(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(playlist_name): extract::Path<String>,
) -> Response {
    let mut state = state.lock().unwrap();

    match db::del_playlist(&mut state.db_conn, playlist_name) {
        Ok(Some(_)) => (
            StatusCode::OK,
            Json(Status {
                status: "ok".into(),
                error: None,
            }),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(Status {
                status: "error".into(),
                error: Some("Playlist not found".into()),
            }),
        )
            .into_response(),
        Err(e) => {
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
async fn list_schedules(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock().unwrap();

    match db::get_schedules(&mut state.db_conn) {
        Ok(schedules) => {
            let schedules: Vec<Schedule> = schedules
                .into_iter()
                .filter_map(|s| s.try_into().ok())
                .collect();
            Json(schedules).into_response()
        }
        Err(e) => {
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
async fn get_schedule(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(schedule): extract::Path<String>,
) -> Response {
    let mut state = state.lock().unwrap();

    match db::get_schedule(&mut state.db_conn, schedule) {
        Ok(Some(s)) => match Schedule::try_from(s) {
            Ok(schedule) => (StatusCode::OK, Json(schedule)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Status {
                    status: "error".into(),
                    error: Some(e.to_string()),
                }),
            )
                .into_response(),
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(Status {
                status: "error".into(),
                error: Some("Schedule not found".into()),
            }),
        )
            .into_response(),
        Err(e) => {
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
async fn new_schedule(
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
async fn update_schedule(
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
async fn del_schedule(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(schedule): extract::Path<String>,
) -> Response {
    let mut state = state.lock().unwrap();

    match db::del_schedule(&mut state.db_conn, schedule) {
        Ok(Some(_)) => (
            StatusCode::OK,
            Json(Status {
                status: "ok".into(),
                error: None,
            }),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(Status {
                status: "error".into(),
                error: Some("Schedule not found".into()),
            }),
        )
            .into_response(),
        Err(e) => {
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

/// Get system info.
///
/// Get the high-level system information. This endpoint is used
/// to simulate FPP and make us discoverable by other software such
/// as xLights. Some values are hard-coded to ensure compatibility.
#[utoipa::path(
    get,
    path = "/api/system/info",
    responses(
        (status = 200, description = "", body = SystemInfo)
    ),
    tag = "FPP Compatibility"
)]
async fn system_info(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let state = state.lock().unwrap();

    let uname = rustix::system::uname();

    let hostname = String::from_utf8_lossy(uname.nodename().to_bytes()).to_string();
    let release = String::from_utf8_lossy(uname.release().to_bytes()).to_string();

    let mut os_release = "Unknown".to_string();
    let mut variant = "Unknown".to_string();

    if let Ok(conf) = ini::Ini::load_from_file("/etc/os-release") {
        if let Some(section) = conf.section(None::<String>) {
            if let Some(value) = section.get("ID") {
                variant = value.to_string();
            }
            if let Some(value) = section.get("PRETTY_NAME") {
                os_release = value.to_string();
            }
        }
    };

    let mut ips = Vec::new();
    if let Ok(network_interfaces) = local_ip_address::list_afinet_netifas() {
        network_interfaces.iter().for_each(|(_name, ip)| {
            ips.push(ip.to_string());
        });
    }

    let mut utilization = SystemUtilization::default();

    let sys = systemstat::System::new();
    if let Ok(load) = sys.load_average() {
        utilization.cpu = load.one;
    }
    if let Ok(mem) = sys.memory() {
        utilization.memory = mem.free.as_u64() as f32 / mem.total.as_u64() as f32;
    }
    if let Ok(uptime) = sys.uptime() {
        utilization.uptime = format!("{}", uptime.human(humanize_duration::Truncate::Second));
    }

    let storage: String = match std::path::absolute(Path::new(&state.cfg.storage)) {
        Ok(p) => p.to_string_lossy().into(),
        Err(_) => "/".into(),
    };

    if let Ok(mounts) = sys.mounts() {
        for mount in mounts.iter() {
            if mount.fs_mounted_on == "/" {
                utilization.disk.root.total = mount.total.as_u64();
                utilization.disk.root.free = mount.avail.as_u64();
            }
            if storage.starts_with(&mount.fs_mounted_on) {
                utilization.disk.media.total = mount.total.as_u64();
                utilization.disk.media.free = mount.avail.as_u64();
            }
        }
    }

    Json(SystemInfo {
        hostname,
        platform: "Linux".to_string(),
        variant,
        mode: "player".to_string(),
        version: "6.0".to_string(),
        branch: built_info::GIT_HEAD_REF.unwrap_or("unknown").into(),
        os_release,
        kernel: release,
        local_git_version: built_info::GIT_VERSION.unwrap_or("unknown").into(),
        ips,
        type_id: 0x01,
        utilization,
        ..Default::default()
    })
    .into_response()
}

/// Retrieve models.json
///
/// Download the models in JSON format
#[utoipa::path(
    get,
    path = "/api/models",
    responses(
        (status = 200, description = "Models downloaded successfully", body = Vec<Model>),
        (status = 404, description = "Models not found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "FPP Compatibility"
)]
async fn get_models(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    download_other(state, "models.json".into(), "application/json".into()).await
}

/// Upload models.json
///
/// Upload the models in JSON format
#[utoipa::path(
    post,
    path = "/api/models",
    request_body(content = Models),
    responses(
        (status = 200, description = "Models uploaded successfully", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "FPP Compatibility"
)]
async fn upload_models(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(models): Json<Models>,
) -> Response {
    match serde_json::to_vec(&models.models) {
        Ok(data) => upload_other(state, "models.json".into(), data).await,
        Err(e) => {
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

/// Retrieve VirtualDisplayMap
///
/// Download the VirtualDisplayMap
#[utoipa::path(
    get,
    path = "/api/configfile/virtualdisplaymap",
    responses(
        (
            status = 200,
            description = "VirtualDisplayMap downloaded successfully",
            content_type = "text/plain",
            body = String
        ),
        (status = 404, description = "VirtualDisplayMap not found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "FPP Compatibility"
)]
async fn get_display(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    download_other(state, "virtual_display_map".into(), "text/plain".into()).await
}

/// Upload VirtualDisplayMap
///
/// Upload the VirtualDisplayMap
#[utoipa::path(
    post,
    path = "/api/configfile/virtualdisplaymap",
    request_body(content = Vec<u8>),
    responses(
        (status = 200, description = "VDM uploaded successfully", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "FPP Compatibility"
)]
async fn upload_display(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    data: Bytes,
) -> Response {
    upload_other(state, "virtual_display_map".into(), data.to_vec()).await
}

/// Retrieve outputs.json
///
/// Download the outputs in JSON format
#[utoipa::path(
    get,
    path = "/api/channel/output/universeOutputs",
    responses(
        (status = 200, description = "Ouputs downloaded successfully", body = Channels),
        (status = 404, description = "Outputs not found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "FPP Compatibility"
)]
async fn get_outputs(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    download_other(state, "outputs.json".into(), "application/json".into()).await
}

/// Upload outputs.json
///
/// Upload the outputs in JSON format
#[utoipa::path(
    post,
    path = "/api/channel/output/universeOutputs",
    request_body(content = Channels),
    responses(
        (status = 200, description = "Outputs uploaded successfully", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "FPP Compatibility"
)]
async fn upload_outputs(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(outputs): Json<Channels>,
) -> Response {
    match serde_json::to_vec(&outputs) {
        Ok(data) => upload_other(state, "outputs.json".into(), data).await,
        Err(e) => {
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
async fn get_scheduler_status(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
) -> Response {
    let state = state.lock().unwrap();

    (
        StatusCode::OK,
        Json(SchedulerStatus {
            status: state.player_state.clone(),
        }),
    )
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
async fn start_scheduler(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let ctrl;
    {
        let state = state.lock().unwrap();
        ctrl = state.player_ctrl.clone();
    }
    if let Err(e) = ctrl.send(PlayerState::Start).await {
        tracing::error!("Could not start scheduler: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Status {
                status: "error".into(),
                error: Some(e.to_string()),
            }),
        )
            .into_response();
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
async fn stop_scheduler(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let ctrl;
    {
        let state = state.lock().unwrap();
        ctrl = state.player_ctrl.clone();
    }

    if let Err(e) = ctrl.send(PlayerState::Stop).await {
        tracing::error!("Could not stop scheduler: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Status {
                status: "error".into(),
                error: Some(e.to_string()),
            }),
        )
            .into_response();
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

async fn create_or_update_playlist(state: Arc<Mutex<State>>, playlist: Playlist) -> Response {
    let playlist_name = match playlist.name {
        Some(n) => n,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Status {
                    status: "error".into(),
                    error: Some("Missing 'name' field".into()),
                }),
            )
                .into_response()
        }
    };

    if playlist_name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Status {
                status: "error".into(),
                error: Some("Name cannot be empty".into()),
            }),
        )
            .into_response();
    }

    let mut state = state.lock().unwrap();

    let sequences = playlist
        .main_playlist
        .into_iter()
        .map(|s| NewSequencePlus {
            enabled: s.enabled,
            play_once: s.play_once,
            sequence: s.sequence_name,
        })
        .collect();

    let new_playlist = db::models::NewPlaylist {
        name: playlist_name,
        description: playlist.desc,
        repeat: playlist.repeat,
        loop_count: playlist.loop_count,
    };

    match db::new_playlist(&mut state.db_conn, (new_playlist, sequences)) {
        Ok(_) => (
            StatusCode::OK,
            Json(Status {
                status: "ok".into(),
                error: None,
            }),
        )
            .into_response(),
        Err(e) => {
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

async fn create_or_update_schedule(state: Arc<Mutex<State>>, schedule: Schedule) -> Response {
    if schedule.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Status {
                status: "error".into(),
                error: Some("Name cannot be empty".into()),
            }),
        )
            .into_response();
    }

    let mut state = state.lock().unwrap();

    let start_date = match chrono::NaiveDate::parse_from_str(&schedule.start_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Status {
                    status: "error".into(),
                    error: Some("Start date isn't a valid date".into()),
                }),
            )
                .into_response()
        }
    }
    .num_days_from_ce();

    let end_date = match chrono::NaiveDate::parse_from_str(&schedule.end_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Status {
                    status: "error".into(),
                    error: Some("End date isn't a valid date".into()),
                }),
            )
                .into_response()
        }
    }
    .num_days_from_ce();

    let start_time = match chrono::NaiveTime::parse_from_str(&schedule.start_time, "%H:%M") {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Status {
                    status: "error".into(),
                    error: Some("Start time isn't a valid time".into()),
                }),
            )
                .into_response()
        }
    }
    .num_seconds_from_midnight();

    let end_time = match chrono::NaiveTime::parse_from_str(&schedule.end_time, "%H:%M") {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Status {
                    status: "error".into(),
                    error: Some("End time isn't a valid time".into()),
                }),
            )
                .into_response()
        }
    }
    .num_seconds_from_midnight();

    let schedule = db::models::NewSchedule {
        name: schedule.name,
        playlist_id: schedule.playlist_id,
        enabled: schedule.enabled,
        start_date,
        end_date,
        start_time: start_time as i64,
        end_time: end_time as i64,
        monday: schedule.monday,
        tuesday: schedule.tuesday,
        wednesday: schedule.wednesday,
        thursday: schedule.thursday,
        friday: schedule.friday,
        saturday: schedule.saturday,
        sunday: schedule.sunday,
    };

    match db::new_schedule(&mut state.db_conn, schedule) {
        Ok(_) => (
            StatusCode::OK,
            Json(Status {
                status: "ok".into(),
                error: None,
            }),
        )
            .into_response(),
        Err(e) => {
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

async fn upload_other(state: Arc<Mutex<State>>, filename: String, data: Vec<u8>) -> Response {
    let state = state.lock().unwrap();

    match storage::upload_file(&state.cfg, &filename, StorageType::Other, data) {
        Ok(_) => (
            StatusCode::OK,
            Json(Status {
                status: "ok".into(),
                error: None,
            }),
        )
            .into_response(),
        Err(e) => {
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

async fn download_other(state: Arc<Mutex<State>>, filename: String, mimetype: String) -> Response {
    let state = state.lock().unwrap();

    match storage::read_file(&state.cfg, &filename, StorageType::Other) {
        Ok(Some(d)) => (StatusCode::OK, [(header::CONTENT_TYPE, mimetype)], d).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(Status {
                status: "error".into(),
                error: Some("models.json not found".into()),
            }),
        )
            .into_response(),
        Err(e) => {
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
