use std::sync::Arc;

use axum::{
    extract,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{
    db::{
        self,
        models::{NewScene, Scene},
    },
    models::Status,
    state::State,
    web::error::APIError,
};

/// List scenes
///
/// List all scenes
#[utoipa::path(
    get,
    path = "/api/scenes",
    responses(
        (status = 200, description = "List of available scenes", body = [Scene]),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scenes"
)]
pub async fn list_scenes(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock();

    match db::get_scenes(&mut state.db_conn) {
        Ok(scenes) => Json(scenes).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Get a scene
///
/// Read a single scene
#[utoipa::path(
    get,
    path = "/api/scenes/{scene}",
    params(
        ("scene" = String, Path, description = "The name of the scene")
    ),
    responses(
        (status = 200, description = "The requested scene", body = Scene),
        (status = 404, description = "The scene wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scenes"
)]
pub async fn get_scene(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(scene): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    match db::get_scene(&mut state.db_conn, scene) {
        Ok(Some(s)) => Json(s).into_response(),
        Ok(None) => APIError::NotFound("Scene".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// New scene
///
/// Create a new scene
#[utoipa::path(
    post,
    path = "/api/scene",
    request_body(content = NewScene),
    responses(
        (status = 201, description = "The scene was created", body = Status),
        (status = 422, description = "Incomplete scene supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scenes"
)]
pub async fn new_scene(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(scene): Json<NewScene>,
) -> Response {
    create_or_update_scene(state, scene).await
}

/// Update a scene
///
/// Create or update the given scene
#[utoipa::path(
    put,
    path = "/api/scenes/{scene}",
    params(
        ("scene" = String, Path, description = "The name of the scene")
    ),
    request_body(content = NewScene),
    responses(
        (status = 200, description = "The requested scene was updated", body = Status),
        (status = 422, description = "Incomplete scene supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scenes"
)]
pub async fn update_scene(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(scene_name): extract::Path<String>,
    Json(mut scene): Json<NewScene>,
) -> Response {
    scene.name = scene_name;
    create_or_update_scene(state, scene).await
}

/// Delete a scene
///
/// Delete the given scene
#[utoipa::path(
    delete,
    path = "/api/scene/{scene}",
    params(
        ("scene" = String, Path, description = "The name of the scene")
    ),
    responses(
        (status = 200, description = "The requested scene was deleted", body = Status),
        (status = 404, description = "The scene wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Scenes"
)]
pub async fn del_scene(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(scene): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    match db::del_scene(&mut state.db_conn, scene) {
        Ok(Some(_)) => APIError::Ok.into_response(),
        Ok(None) => APIError::NotFound("Scene".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

async fn create_or_update_scene(state: Arc<Mutex<State>>, scene: NewScene) -> Response {
    if scene.name.is_empty() {
        return APIError::BadRequest("Name cannot be empty".into()).into_response();
    }

    let mut state = state.lock();

    match db::new_scene(&mut state.db_conn, scene) {
        Ok(_) => APIError::Ok.into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}
