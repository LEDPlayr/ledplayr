use std::sync::Arc;

use axum::{
    extract,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{
    db::{self, models::NewSequencePlus},
    models::*,
    state::State,
    web::error::APIError,
};

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
pub async fn list_playlists(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock();

    match db::get_playlists(&mut state.db_conn) {
        Ok(playlists) => {
            let ret: Vec<String> = playlists.iter().map(|p| p.name.clone()).collect();
            Json(ret).into_response()
        }
        Err(e) => APIError::UnexpectedError(e).into_response(),
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
pub async fn list_playlists_numbered(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
) -> Response {
    let mut state = state.lock();

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
        Err(e) => APIError::UnexpectedError(e).into_response(),
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
pub async fn get_playlist(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(playlist): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

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
            })
            .into_response()
        }
        Ok(None) => APIError::BadRequest("Playlist not found".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
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
pub async fn new_playlist(
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
pub async fn update_playlist(
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
pub async fn del_playlist(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(playlist_name): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    match db::del_playlist(&mut state.db_conn, playlist_name) {
        Ok(Some(_)) => APIError::Ok.into_response(),
        Ok(None) => APIError::BadRequest("Playlist not found".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

async fn create_or_update_playlist(state: Arc<Mutex<State>>, playlist: Playlist) -> Response {
    let playlist_name = match playlist.name {
        Some(n) => n,
        None => return APIError::BadRequest("Missing 'name' field".into()).into_response(),
    };

    if playlist_name.is_empty() {
        return APIError::BadRequest("Name cannot be empty".into()).into_response();
    }

    let mut state = state.lock();

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
        Ok(Some(_)) => APIError::Ok.into_response(),
        Ok(None) => APIError::NotFound("Playlist/Sequence".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}
