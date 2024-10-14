use std::net::Ipv4Addr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LogPeriod {
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
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
