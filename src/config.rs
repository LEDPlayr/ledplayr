use std::net::Ipv4Addr;

use serde::Deserialize;

/// App Config
#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub multicast: Option<bool>,
    pub storage: String,

    pub web: Option<WebConfig>,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub bind: Option<Ipv4Addr>,
    pub port: Option<u16>,
}
