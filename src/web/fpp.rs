use std::{path::Path, sync::Arc};

use axum::{
    body::Bytes,
    extract,
    response::{IntoResponse, Response},
    Json,
};
use humanize_duration::prelude::DurationExt;
use parking_lot::Mutex;
use rustix::path::Arg;
use systemstat::{saturating_sub_bytes, Platform};

use crate::{
    built_info,
    models::*,
    state::State,
    storage,
    web::{error::APIError, utils},
};

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
pub async fn fpp_command(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    query: extract::Query<CommandQuery>,
) -> Response {
    let state = state.lock();

    match query.command.as_str() {
        "moveFile" => {
            let dir = match storage::get_dir(&query.file) {
                Some(d) => d,
                None => {
                    return APIError::BadRequest("Unrecognized file type".into()).into_response()
                }
            };

            let path = Path::new(&state.cfg.storage)
                .join(dir.to_string())
                .join(&query.file);
            if path.exists() {
                APIError::Ok.into_response()
            } else {
                APIError::NotFound("File".into()).into_response()
            }
        }
        _ => APIError::BadRequest("Unrecognized command".into()).into_response(),
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
pub async fn system_info(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    let state = state.lock();

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
        utilization.memory =
            saturating_sub_bytes(mem.total, mem.free).as_u64() as f32 / mem.total.as_u64() as f32;
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
pub async fn list_models(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    utils::download_other(state, "models.json".into(), "application/json".into()).await
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
pub async fn upload_models(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(models): Json<Models>,
) -> Response {
    match serde_json::to_vec(&models.models) {
        Ok(data) => utils::upload_other(state, "models.json".into(), data).await,
        Err(e) => APIError::UnexpectedError(e.into()).into_response(),
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
pub async fn get_display(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    utils::download_other(state, "virtual_display_map".into(), "text/plain".into()).await
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
pub async fn upload_display(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    data: Bytes,
) -> Response {
    utils::upload_other(state, "virtual_display_map".into(), data.to_vec()).await
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
pub async fn get_outputs(extract::State(state): extract::State<Arc<Mutex<State>>>) -> Response {
    utils::download_other(state, "outputs.json".into(), "application/json".into()).await
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
pub async fn upload_outputs(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(outputs): Json<Channels>,
) -> Response {
    match serde_json::to_vec(&outputs) {
        Ok(data) => utils::upload_other(state, "outputs.json".into(), data).await,
        Err(e) => APIError::UnexpectedError(e.into()).into_response(),
    }
}
