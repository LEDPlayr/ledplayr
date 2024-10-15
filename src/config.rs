use std::net::Ipv4Addr;

use serde::Deserialize;
use time::format_description;

#[derive(Debug, Deserialize)]
pub enum LogPeriod {
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "never")]
    Never,
}

impl LogPeriod {
    // https://docs.rs/tracing-appender/latest/src/tracing_appender/rolling.rs.html#495
    pub fn date_format(&self) -> Vec<format_description::FormatItem<'static>> {
        match *self {
            LogPeriod::Minute => format_description::parse("[year]-[month]-[day]-[hour]-[minute]"),
            LogPeriod::Hour => format_description::parse("[year]-[month]-[day]-[hour]"),
            LogPeriod::Day => format_description::parse("[year]-[month]-[day]"),
            LogPeriod::Never => format_description::parse("[year]-[month]-[day]"),
        }
        .expect("Unable to create a formatter; this is a bug in tracing-appender")
    }
}

/// App Config
#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub multicast: Option<bool>,
    pub storage: String,

    pub web: Option<WebConfig>,
    pub log: Option<LogConfig>,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub bind: Option<Ipv4Addr>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub directory: String,
    pub prefix: Option<String>,
    pub period: Option<LogPeriod>,
    pub max_files: Option<usize>,
}
