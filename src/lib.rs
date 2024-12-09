pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub mod button;
pub mod config;
pub mod db;
pub mod error;
pub mod fpp;
pub mod fseq;
pub mod models;
pub mod patterns;
pub mod player;
pub mod state;
pub mod storage;
pub mod web;
