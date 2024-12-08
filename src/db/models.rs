use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    query_builder::AsChangeset,
    Selectable,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::schema;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Serialize, ToSchema)]
#[diesel(table_name = schema::buttons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Button {
    pub id: i32,
    pub status: String,
    pub error: String,
    pub battery: f32,
    pub input: bool,
    pub last: i32,
    pub now: i32,
}

#[derive(Insertable, PartialEq, Debug, AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = schema::buttons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewButton {
    pub status: String,
    pub error: String,
    pub battery: f32,
    pub input: bool,
    pub last: i32,
    pub now: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Serialize, ToSchema)]
#[diesel(table_name = schema::meshes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Mesh {
    pub id: i32,
    pub name: String,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
}

#[derive(Insertable, PartialEq, Debug, AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = schema::meshes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewMesh {
    pub name: String,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = schema::playlists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub repeat: bool,
    pub loop_count: i32,
}

#[derive(Insertable, PartialEq, Debug, AsChangeset)]
#[diesel(table_name = schema::playlists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewPlaylist {
    pub name: String,
    pub description: String,
    pub repeat: bool,
    pub loop_count: i32,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = schema::playlists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdatePlaylist {
    pub name: Option<String>,
    pub description: Option<String>,
    pub repeat: Option<bool>,
    pub loop_count: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Serialize, Debug, ToSchema)]
#[diesel(table_name = schema::scenes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Scene {
    pub id: i32,
    pub name: String,
    pub cam_pos_x: f32,
    pub cam_pos_y: f32,
    pub cam_pos_z: f32,
    pub cam_rot_x: f32,
    pub cam_rot_y: f32,
    pub cam_rot_z: f32,
    pub cam_zoom: f32,
    pub ctrl_x: f32,
    pub ctrl_y: f32,
    pub ctrl_z: f32,
}

#[derive(Insertable, PartialEq, Debug, AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = schema::scenes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewScene {
    pub name: String,
    pub cam_pos_x: f32,
    pub cam_pos_y: f32,
    pub cam_pos_z: f32,
    pub cam_rot_x: f32,
    pub cam_rot_y: f32,
    pub cam_rot_z: f32,
    pub cam_zoom: f32,
    pub ctrl_x: f32,
    pub ctrl_y: f32,
    pub ctrl_z: f32,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = schema::sequences)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sequence {
    pub id: i32,
    pub name: String,
    pub timestamp: String,
    pub step_time: i32,
    pub frames: i32,
    pub channels: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(Playlist))]
#[diesel(table_name = schema::schedules)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Schedule {
    pub id: i32,
    pub name: String,
    pub playlist_id: i32,
    pub enabled: bool,

    pub start_date: i32,
    pub end_date: i32,
    pub start_time: i64,
    pub end_time: i64,

    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(Sequence))]
#[diesel(table_name = schema::variables)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Variable {
    pub id: i32,
    pub sequence_id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Insertable, Identifiable, Selectable, Queryable, Associations, Debug, AsChangeset)]
#[diesel(belongs_to(Playlist))]
#[diesel(belongs_to(Sequence))]
#[diesel(table_name = schema::playlists_sequences)]
#[diesel(primary_key(playlist_id, sequence_id))]
pub struct PlaylistSequence {
    pub playlist_id: i32,
    pub sequence_id: i32,
    pub sort_by: i32,
    pub enabled: bool,
    pub play_once: bool,
}

#[derive(Insertable, PartialEq, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = schema::sequences)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSequence {
    pub name: String,
    pub timestamp: String,
    pub step_time: i32,
    pub frames: i32,
    pub channels: i32,
}

#[derive(Insertable, PartialEq, Debug, AsChangeset)]
#[diesel(table_name = schema::schedules)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSchedule {
    pub name: String,
    pub playlist_id: i32,
    pub enabled: bool,

    pub start_date: i32,
    pub end_date: i32,
    pub start_time: i64,
    pub end_time: i64,

    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = schema::variables)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewVariable {
    pub sequence_id: i32,
    pub name: String,
    pub value: String,
}

pub struct SequencePlus {
    pub enabled: bool,
    pub play_once: bool,
    pub sequence: Sequence,
}

pub struct NewSequencePlus {
    pub enabled: bool,
    pub play_once: bool,
    pub sequence: String,
}

pub type PlaylistAndSeq = (Playlist, Vec<SequencePlus>);
pub type NewPlaylistAndSeq = (NewPlaylist, Vec<NewSequencePlus>);
pub type NextSchedule = (Schedule, Playlist, Vec<(bool, Sequence)>);
