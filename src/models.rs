use std::{collections::HashMap, net::Ipv4Addr};

use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::{db, patterns::TestSpec};

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct SystemInfo {
    /// Hostname of the system
    #[schema(example = "localhost")]
    #[serde(rename = "HostName")]
    pub hostname: String,
    /// The description of this system
    #[schema(example = "Rust based FPP alternative")]
    #[serde(rename = "HostDescription")]
    pub host_description: String,
    /// The platform we're running on
    #[schema(example = "Linux")]
    #[serde(rename = "Platform")]
    pub platform: String,
    /// The platform we're running on
    #[schema(example = "Debian")]
    #[serde(rename = "Variant")]
    pub variant: String,
    /// Any additional contex to the platform
    #[schema(example = "")]
    #[serde(rename = "SubPlatform")]
    pub sub_platform: String,
    /// The background color to use in the UI
    #[schema(example = "#c01015")]
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    /// The mode of this system
    #[schema(example = "player")]
    #[serde(rename = "Mode")]
    pub mode: String,
    /// The logo of this system
    #[schema(example = "debian.png")]
    #[serde(rename = "Logo")]
    pub logo: String,
    /// The version of "FPPF" we're runnnig
    #[schema(example = "6.0")]
    #[serde(rename = "Version")]
    pub version: String,
    /// The CVS branch we're running on
    #[schema(example = "main")]
    #[serde(rename = "Branch")]
    pub branch: String,
    /// Is multisync supported
    #[schema(example = false)]
    pub multisync: bool,
    /// The OS Version
    #[schema(example = "Stretch")]
    #[serde(rename = "OSVersion")]
    pub os_version: String,
    /// The OS Release
    #[schema(example = "")]
    #[serde(rename = "OSRelease")]
    pub os_release: String,
    /// The persistent UUID for this system
    #[schema(example = "82ae0c57-9a54-4911-9dc2-a1d2e512da7b")]
    pub uuid: String,
    /// The current system utilization
    #[serde(rename = "Utilization")]
    pub utilization: SystemUtilization,
    /// The kernel version
    #[schema(example = "6.4.4")]
    #[serde(rename = "Kernel")]
    pub kernel: String,
    /// The version for CVS
    #[schema(example = "1.0.0")]
    #[serde(rename = "LocalGitVersion")]
    pub local_git_version: String,
    /// The latest upstream CVS version
    #[schema(example = "1.0.0")]
    #[serde(rename = "RemoteGitVersion")]
    pub remote_git_version: String,
    /// The location of updates
    #[schema(example = "git")]
    #[serde(rename = "UpgradeSource")]
    pub upgrade_source: String,
    /// The IP addresses for this host
    #[schema(example = json!(["127.0.0.1"]))]
    #[serde(rename = "IPs")]
    pub ips: Vec<String>,
    /// The type of system
    #[schema(example = 0x01)]
    #[serde(rename = "typeId")]
    pub type_id: u8,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct SystemUtilization {
    #[serde(rename = "CPU")]
    pub cpu: f32,
    #[serde(rename = "Memory")]
    pub memory: f32,
    #[serde(rename = "Uptime")]
    pub uptime: String,
    #[serde(rename = "Disk")]
    pub disk: DiskUtilization,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct DiskUtilization {
    #[serde(rename = "Media")]
    pub media: FreeTotal,
    #[serde(rename = "Root")]
    pub root: FreeTotal,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct FreeTotal {
    #[serde(rename = "Free")]
    pub free: u64,
    #[serde(rename = "Total")]
    pub total: u64,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct SequenceMeta {
    /// Name of the sequence
    #[schema(example = "sequence.fseq")]
    #[serde(rename = "Name")]
    pub name: String,
    /// ID of the sequence (Likely the creation timestamp)
    #[schema(example = "12345")]
    #[serde(rename = "ID")]
    pub id: String,
    /// Step time in milliseconds
    #[schema(example = 50)]
    #[serde(rename = "StepTime")]
    pub step_time: u8,
    /// Number of framess
    #[schema(example = 100)]
    #[serde(rename = "NumFrames")]
    pub num_frames: u32,
    /// Number of channels
    #[schema(example = 10)]
    #[serde(rename = "ChannelCount")]
    pub channel_count: u32,
    /// Any additional variables
    #[schema(example = json!({"sp": "xLights"}))]
    #[serde(rename = "variableHeaders")]
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct Status {
    /// Status
    #[schema(example = "error")]
    pub status: String,
    /// What went wrong
    #[schema(example = "Could not open file")]
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct CommandQuery {
    pub command: String,
    pub file: String,
}

#[derive(TryFromMultipart, ToSchema)]
pub struct FileUpload {
    /// File or files to upload
    #[form_data(limit = "2MiB")]
    #[schema(value_type = String, format = Binary)]
    pub myfile: Vec<FieldData<Bytes>>,
}

#[derive(ToSchema)]
#[schema(value_type = String, format = Binary)]
pub struct BinaryFile(pub Vec<u8>);

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Playlist {
    #[schema(example = "sample")]
    pub name: Option<String>,
    #[schema(example = 3)]
    pub version: u8,
    #[schema(example = false)]
    pub repeat: bool,
    #[schema(example = 0)]
    #[serde(rename = "loopCount")]
    pub loop_count: i32,
    #[schema(example = true)]
    pub empty: bool,
    #[schema(example = "")]
    pub desc: String,
    #[schema(example = false)]
    pub random: bool,
    #[serde(rename = "leadIn")]
    pub lead_in: Vec<PlaylistEntry>,
    #[serde(rename = "mainPlaylist")]
    pub main_playlist: Vec<PlaylistEntry>,
    #[serde(rename = "leadOut")]
    pub lead_out: Vec<PlaylistEntry>,
    #[serde(rename = "playlistInfo")]
    pub playlist_info: Option<PlaylistInfo>,
}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PlaylistInfo {
    #[schema(example = 0)]
    pub total_duration: f32,
    #[schema(example = 0)]
    pub total_items: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlaylistEntry {
    #[schema(example = 30)]
    pub duration: Option<f32>,
    #[schema(example = "sequence.fseq")]
    #[serde(rename = "sequenceName")]
    pub sequence_name: String,
    #[schema(example = false)]
    #[serde(rename = "playOnce", deserialize_with = "boolean")]
    pub play_once: bool,
    #[schema(example = true)]
    #[serde(deserialize_with = "boolean")]
    pub enabled: bool,
    #[schema(example = "sequence")]
    #[serde(rename = "type")]
    pub playlist_type: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Model {
    #[schema(example = "Single_line")]
    #[serde(rename = "Name")]
    pub name: String,
    #[schema(example = true)]
    #[serde(rename = "xLights")]
    pub x_lights: bool,
    #[schema(example = 6)]
    #[serde(rename = "ChannelCount")]
    pub channel_count: u32,
    #[schema(example = "horizontal")]
    #[serde(rename = "Orientation")]
    pub orientation: String,
    #[schema(example = 1)]
    #[serde(rename = "StartChannel")]
    pub start_channel: u32,
    #[schema(example = 1)]
    #[serde(rename = "StringCount")]
    pub string_count: u32,
    #[schema(example = 3)]
    #[serde(rename = "ChannelCountPerNode")]
    pub channel_count_per_node: u8,
    #[schema(example = 1)]
    #[serde(rename = "StrandsPerString")]
    pub strands_per_string: u8,
    #[schema(example = "BL")]
    #[serde(rename = "StartCorner")]
    pub start_corner: String,
    #[schema(example = "Channel")]
    #[serde(rename = "Type")]
    pub model_type: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Models {
    pub models: Vec<Model>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Universe {
    pub description: String,
    #[serde(deserialize_with = "boolean")]
    pub active: bool,
    #[schema(value_type = String, format = "ipv4")]
    pub address: Ipv4Addr,
    #[serde(rename = "startChannel")]
    pub start_channel: u32,
    #[serde(rename = "channelCount")]
    pub channel_count: u32,
    pub id: u32,
    #[serde(rename = "deDuplicate", deserialize_with = "boolean")]
    pub de_duplicate: bool,
    pub priority: u8,
    #[serde(deserialize_with = "boolean")]
    pub monitor: bool,
    #[serde(rename = "type")]
    pub universe_type: u8,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ChannelOutput {
    #[serde(rename = "type")]
    pub channel_type: String,
    #[serde(rename = "startChannel")]
    pub start_channel: u32,
    #[serde(deserialize_with = "boolean")]
    pub enabled: bool,
    pub timeout: u32,
    #[serde(rename = "channelCount")]
    pub channel_count: i32,
    pub universes: Vec<Universe>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Channels {
    #[serde(rename = "channelOutputs")]
    pub channel_outputs: Vec<ChannelOutput>,
}

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlayerState {
    Schedule,
    Playlist(String),
    Sequence(String),
    Test(TestSpec),
    Stop,
}

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlayerStatus {
    Scheduler,
    Playlist,
    Sequence,
    Testing,
    Stopped,
}

#[derive(Debug, PartialEq, Serialize, Clone, ToSchema)]
pub struct NumberedPlaylist {
    #[schema(example = "name")]
    pub name: String,
    #[schema(example = 1)]
    pub id: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, ToSchema)]
pub struct Schedule {
    #[schema(example = "Schedule")]
    pub name: String,
    #[schema(example = 1)]
    pub playlist_id: i32,
    #[schema(example = true)]
    pub enabled: bool,

    #[schema(example = "1920-01-01", format = "date")]
    pub start_date: String,
    #[schema(example = "1970-01-01", format = "date")]
    pub end_date: String,
    #[schema(example = "00:00", format = "time")]
    pub start_time: String,
    #[schema(example = "00:00", format = "time")]
    pub end_time: String,

    #[schema(example = true)]
    pub monday: bool,
    #[schema(example = true)]
    pub tuesday: bool,
    #[schema(example = true)]
    pub wednesday: bool,
    #[schema(example = true)]
    pub thursday: bool,
    #[schema(example = true)]
    pub friday: bool,
    #[schema(example = true)]
    pub saturday: bool,
    #[schema(example = true)]
    pub sunday: bool,
}

fn boolean<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        Value::Bool(b) => b,
        Value::String(s) => s == "yes",
        Value::Number(num) => {
            num.as_i64()
                .ok_or(serde::de::Error::custom("Invalid number"))?
                != 0
        }
        Value::Null => false,
        _ => return Err(serde::de::Error::custom("Wrong type, expected boolean")),
    })
}

impl TryFrom<db::models::Schedule> for Schedule {
    type Error = String;

    fn try_from(value: db::models::Schedule) -> Result<Self, Self::Error> {
        let start_date = match chrono::NaiveDate::from_num_days_from_ce_opt(value.start_date) {
            Some(s) => s,
            None => {
                tracing::error!("Could not convert start_date");
                return Err("Could not convert start_date".into());
            }
        }
        .format("%Y-%m-%d")
        .to_string();

        let end_date = match chrono::NaiveDate::from_num_days_from_ce_opt(value.end_date) {
            Some(s) => s,
            None => {
                tracing::error!("Could not convert end_date");
                return Err("Could not convert end_date".into());
            }
        }
        .format("%Y-%m-%d")
        .to_string();

        let start_time =
            match chrono::NaiveTime::from_num_seconds_from_midnight_opt(value.start_time as u32, 0)
            {
                Some(s) => s,
                None => {
                    tracing::error!("Could not convert start_time");
                    return Err("Could not convert start_time".into());
                }
            }
            .format("%H:%M")
            .to_string();

        let end_time =
            match chrono::NaiveTime::from_num_seconds_from_midnight_opt(value.end_time as u32, 0) {
                Some(s) => s,
                None => {
                    tracing::error!("Could not convert end_time");
                    return Err("Could not convert end_time".into());
                }
            }
            .format("%H:%M")
            .to_string();

        Ok(Schedule {
            name: value.name,
            playlist_id: value.playlist_id,
            enabled: value.enabled,
            start_date,
            end_date,
            start_time,
            end_time,
            monday: value.monday,
            tuesday: value.tuesday,
            wednesday: value.wednesday,
            thursday: value.thursday,
            friday: value.friday,
            saturday: value.saturday,
            sunday: value.sunday,
        })
    }
}

impl TryFrom<Schedule> for db::models::NewSchedule {
    type Error = String;

    fn try_from(value: Schedule) -> Result<Self, Self::Error> {
        let start_date = match chrono::NaiveDate::parse_from_str(&value.start_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => return Err("Start date isn't a valid date".into()),
        }
        .num_days_from_ce();

        let end_date = match chrono::NaiveDate::parse_from_str(&value.end_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => return Err("End date isn't a valid date".into()),
        }
        .num_days_from_ce();

        let start_time = match chrono::NaiveTime::parse_from_str(&value.start_time, "%H:%M") {
            Ok(d) => d,
            Err(_) => return Err("Start time isn't a valid time".into()),
        }
        .num_seconds_from_midnight();

        let end_time = match chrono::NaiveTime::parse_from_str(&value.end_time, "%H:%M") {
            Ok(d) => d,
            Err(_) => return Err("End time isn't a valid time".into()),
        }
        .num_seconds_from_midnight();

        Ok(db::models::NewSchedule {
            name: value.name,
            playlist_id: value.playlist_id,
            enabled: value.enabled,
            start_date,
            end_date,
            start_time: start_time as i64,
            end_time: end_time as i64,
            monday: value.monday,
            tuesday: value.tuesday,
            wednesday: value.wednesday,
            thursday: value.thursday,
            friday: value.friday,
            saturday: value.saturday,
            sunday: value.sunday,
        })
    }
}
