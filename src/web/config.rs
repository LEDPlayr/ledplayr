use std::sync::Arc;

use axum::{
    extract,
    response::{IntoResponse, Response},
    Json,
};
use chrono;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::state::State;

use super::error::APIError;

fn get_zoneinfo_base() -> Option<String> {
    let possible_dirs = [
        "/usr/share/zoneinfo",
        "/usr/share/lib/zoneinfo",
        "/usr/lib/zoneinfo",
        "/usr/local/etc/zoneinfo",
        "/etc/zoneinfo",
    ];

    for dir in possible_dirs {
        if std::path::Path::new(dir).exists() {
            return Some(dir.to_string());
        }
    }
    None
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct TimeAndTimezone {
    /// Current server time in RFC3339 format
    time: String,
    /// Current server timezone name (e.g., "Europe/London")
    timezone: String,
}

/// List available timezones
///
/// List all available timezone names
#[utoipa::path(
    get,
    path = "/api/config/timezones",
    responses(
        (status = 200, description = "List of timezones", body = Vec<String>),
        (status = 500, description = "Something went wrong", body = crate::models::Status)
    ),
    tag = "Config"
)]
pub async fn list_timezones(extract::State(_state): extract::State<Arc<Mutex<State>>>) -> Response {
    fn collect_timezones(dir: &std::path::Path, prefix: &str, timezones: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    let full_name = if prefix.is_empty() {
                        name.clone()
                    } else {
                        format!("{}/{}", prefix, name)
                    };
                    if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        timezones.push(full_name);
                    } else if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        collect_timezones(&entry.path(), &full_name, timezones);
                    }
                }
            }
        }
    }

    if let Some(base) = get_zoneinfo_base() {
        let path = std::path::Path::new(&base);
        let mut timezones = Vec::new();
        collect_timezones(path, "", &mut timezones);
        timezones.sort();
        return Json(timezones).into_response();
    }

    APIError::UnexpectedError(anyhow::anyhow!("No timezone directory found")).into_response()
}

/// Set system time and timezone
///
/// Set the server's system time and timezone
#[utoipa::path(
    post,
    path = "/api/config/timezone",
    request_body(content = TimeAndTimezone),
    responses(
        (status = 200, description = "Time and timezone set successfully", body = crate::models::Status),
        (status = 403, description = "Insufficient privileges", body = crate::models::Status),
        (status = 400, description = "Invalid input", body = crate::models::Status),
        (status = 500, description = "Something went wrong", body = crate::models::Status)
    ),
    tag = "Config"
)]
pub async fn set_timezone(Json(config): Json<TimeAndTimezone>) -> Response {
    // Check privileges
    let has_cap = caps::has_cap(
        None,
        caps::CapSet::Effective,
        caps::Capability::CAP_SYS_TIME,
    )
    .unwrap_or(false);
    let is_root = std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false);

    if !has_cap && !is_root {
        return APIError::Forbidden("Insufficient privileges to set system time".into())
            .into_response();
    }

    // Validate timezone
    let base = get_zoneinfo_base();
    if base.is_none() {
        return APIError::Forbidden("Unable to find zoneinfo".into()).into_response();
    }
    let tz_path = format!("{}/{}", base.unwrap(), config.timezone);
    if !std::path::Path::new(&tz_path).exists() {
        return APIError::BadRequest("Invalid timezone".into()).into_response();
    }

    // Parse time
    let time = match chrono::DateTime::parse_from_rfc3339(&config.time) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => return APIError::BadRequest("Invalid time format".into()).into_response(),
    };

    // Set time
    let time_cmd = std::process::Command::new("date")
        .arg("-s")
        .arg(time.format("%Y-%m-%d %H:%M:%S").to_string())
        .output();

    match time_cmd {
        Ok(output) if output.status.success() => {}
        _ => {
            return APIError::UnexpectedError(anyhow::anyhow!("Failed to set time")).into_response()
        }
    }

    // Set timezone
    let tz_cmd = std::process::Command::new("ln")
        .arg("-sf")
        .arg(tz_path)
        .arg("/etc/localtime")
        .output();

    match tz_cmd {
        Ok(output) if output.status.success() => {}
        _ => {
            return APIError::UnexpectedError(anyhow::anyhow!("Failed to set timezone"))
                .into_response()
        }
    }

    APIError::Ok.into_response()
}

/// Get current server time and timezone
///
/// Get the server's current local time and timezone
#[utoipa::path(
    get,
    path = "/api/config/timezone",
    responses(
        (status = 200, description = "Current time and timezone", body = TimeAndTimezone),
        (status = 500, description = "Something went wrong", body = crate::models::Status)
    ),
    tag = "Config"
)]
pub async fn get_current_time_and_timezone() -> Response {
    let now = chrono::Local::now();
    let time = now.to_rfc3339();

    let timezone = if let Ok(link) = std::fs::read_link("/etc/localtime") {
        if let Some(base) = get_zoneinfo_base() {
            if let Ok(relative) = link.strip_prefix(&base) {
                relative.to_string_lossy().to_string()
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };

    Json(TimeAndTimezone { time, timezone }).into_response()
}
