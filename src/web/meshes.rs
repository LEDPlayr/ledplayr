use std::sync::Arc;

use axum::{
    extract::{self},
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;

use crate::{
    db::{self, models::NewMesh},
    state::State,
    storage::{self, StorageType},
    web::error::APIError,
};

/// List all meshes
///
/// List all 3D meshes
#[utoipa::path(
    get,
    path = "/api/meshes",
    responses(
        (status = 200, description = "List of meshes", body = Vec<Mesh>),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Meshes"
)]
pub async fn list_meshes(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let mut state = state.lock();

    match db::get_meshes(&mut state.db_conn) {
        Ok(m) => Json(m).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// Get a 3D mesh
///
/// Download a 3D mesh for the virtual display
#[utoipa::path(
    get,
    path = "/api/mesh/{mesh}",
    params(
        ("mesh" = String, Path, description = "The name of the mesh")
    ),
    responses(
        (status = 200, description = "The requested mesh", body = Vec<u8>),
        (status = 404, description = "Unknown mesh", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Meshes"
)]
pub async fn download_mesh(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(mesh_name): extract::Path<String>,
) -> Response {
    let state = state.lock();

    match storage::read_file(&state.cfg, &mesh_name, StorageType::Meshes) {
        Ok(Some(d)) => ([(header::CONTENT_TYPE, "model/gltf-binary")], d).into_response(),
        Ok(None) => APIError::BadRequest("models.json not found".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

/// New mesh
///
/// Create a new mesh
#[utoipa::path(
    post,
    path = "/api/mesh",
    request_body(content = NewMesh),
    responses(
        (status = 201, description = "The mesh was created", body = Status),
        (status = 422, description = "Incomplete mesh supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Meshes"
)]
pub async fn new_mesh(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(mesh): Json<NewMesh>,
) -> Response {
    create_or_update_mesh(state, mesh).await
}

/// Update a mesh
///
/// Create or update the given mesh
#[utoipa::path(
    put,
    path = "/api/mesh/{mesh}",
    params(
        ("mesh" = String, Path, description = "The name of the mesh")
    ),
    request_body(content = NewMesh),
    responses(
        (status = 200, description = "The requested mesh was updated", body = Status),
        (status = 422, description = "Incomplete mesh supplied", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Meshes"
)]
pub async fn update_mesh(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(mesh_name): extract::Path<String>,
    Json(mut mesh): Json<NewMesh>,
) -> Response {
    mesh.name = mesh_name;
    create_or_update_mesh(state, mesh).await
}

/// Delete a mesh
///
/// Delete the given mesh
#[utoipa::path(
    delete,
    path = "/api/mesh/{mesh}",
    params(
        ("mesh" = String, Path, description = "The name of the mesh")
    ),
    responses(
        (status = 200, description = "The requested mesh was deleted", body = Status),
        (status = 404, description = "The mesh wasn't found", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Meshes"
)]
pub async fn del_mesh(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    extract::Path(mesh): extract::Path<String>,
) -> Response {
    let mut state = state.lock();

    match db::del_mesh(&mut state.db_conn, mesh) {
        Ok(Some(_)) => APIError::Ok.into_response(),
        Ok(None) => APIError::NotFound("Scene".into()).into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}

async fn create_or_update_mesh(state: Arc<Mutex<State>>, mesh: NewMesh) -> Response {
    if mesh.name.is_empty() {
        return APIError::BadRequest("Name cannot be empty".into()).into_response();
    }

    let mut state = state.lock();

    match db::new_mesh(&mut state.db_conn, mesh) {
        Ok(_) => APIError::Ok.into_response(),
        Err(e) => APIError::UnexpectedError(e).into_response(),
    }
}
