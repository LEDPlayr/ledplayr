use std::path::Path;

use anyhow::{anyhow, bail, Result};
use chrono::{Datelike, Timelike};
use diesel::{
    result::Error::NotFound, sqlite::Sqlite, BelongingToDsl, Connection, ExpressionMethods,
    Insertable, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::{config::Config, fseq};
use models::*;
use schema::*;

pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn get(cfg: &Config) -> Result<SqliteConnection> {
    match SqliteConnection::establish(&cfg.database_url) {
        Ok(connection) => Ok(connection),
        Err(e) => bail!("Error connecting to database: {e}"),
    }
}

pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
    tracing::info!("Running database migrations");

    if let Err(e) = connection.run_pending_migrations(MIGRATIONS) {
        bail!(e.to_string());
    }

    Ok(())
}

pub fn get_sequences(conn: &mut SqliteConnection) -> Result<Vec<Sequence>> {
    sequences::table
        .load::<Sequence>(conn)
        .map_err(|e| anyhow!(e))
}

pub fn get_sequence(
    conn: &mut SqliteConnection,
    n: String,
) -> Result<Option<(Sequence, Vec<Variable>)>> {
    match sequences::table
        .filter(sequences::name.eq(n))
        .select(Sequence::as_select())
        .first(conn)
    {
        Ok(seq) => {
            let vars = Variable::belonging_to(&seq)
                .select(Variable::as_select())
                .load(conn)
                .map_err(|e| anyhow!(e))?;

            Ok(Some((seq, vars)))
        }
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn new_sequence(conn: &mut SqliteConnection, seq: fseq::parser::FSeq) -> Result<()> {
    let filename = match Path::new(&seq.filename).file_name() {
        Some(f) => f.to_string_lossy().to_string(),
        None => "".to_string(),
    };

    let ns = NewSequence {
        name: filename,
        timestamp: seq.uuid.to_string(),
        step_time: seq.step_time_ms as i32,
        frames: seq.frame_count as i32,
        channels: seq.channel_count as i32,
    };

    diesel::insert_into(sequences::table)
        .values(&ns)
        .on_conflict(sequences::name)
        .do_update()
        .set(&ns)
        .execute(conn)?;

    let id = sequences::table
        .select(diesel::dsl::max(sequences::id))
        .first::<Option<i32>>(conn)?;

    if let Some(id) = id {
        for v in seq.variables.into_iter() {
            NewVariable {
                sequence_id: id,
                name: v.code,
                value: v.data,
            }
            .insert_into(variables::table)
            .execute(conn)?;
        }
    }

    Ok(())
}

pub fn del_sequence(conn: &mut SqliteConnection, n: String) -> Result<Option<()>> {
    match sequences::table
        .filter(sequences::name.eq(n))
        .select(sequences::id)
        .first::<i32>(conn)
    {
        Ok(seq_id) => {
            diesel::delete(sequences::table.filter(sequences::id.eq(seq_id))).execute(conn)?;
            diesel::delete(variables::table.filter(variables::sequence_id.eq(seq_id)))
                .execute(conn)?;
            Ok(Some(()))
        }
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn get_playlists(conn: &mut SqliteConnection) -> Result<Vec<Playlist>> {
    playlists::table
        .load::<Playlist>(conn)
        .map_err(|e| anyhow!(e))
}

pub fn get_playlist(
    conn: &mut SqliteConnection,
    playlist: String,
) -> Result<Option<PlaylistAndSeq>> {
    let playlist = match playlists::table
        .filter(playlists::name.eq(playlist))
        .select(Playlist::as_select())
        .first(conn)
    {
        Ok(p) => p,
        Err(NotFound) => return Ok(None),
        Err(e) => return Err(anyhow!(e)),
    };

    let sequences = match playlists_sequences::table
        .filter(playlists_sequences::playlist_id.eq(playlist.id))
        .order_by(playlists_sequences::sort_by.asc())
        .inner_join(sequences::table)
        .select((
            playlists_sequences::enabled,
            playlists_sequences::play_once,
            Sequence::as_select(),
        ))
        .load::<(bool, bool, Sequence)>(conn)
    {
        Ok(v) => v,
        Err(NotFound) => return Ok(None),
        Err(e) => return Err(anyhow!(e)),
    };

    let sequences = sequences
        .into_iter()
        .map(|(enabled, play_once, seq)| SequencePlus {
            enabled,
            play_once,
            sequence: seq,
        })
        .collect();

    Ok(Some((playlist, sequences)))
}

pub fn new_playlist(
    conn: &mut SqliteConnection,
    to_insert: NewPlaylistAndSeq,
) -> Result<Option<()>> {
    let (playlist, seqs) = to_insert;

    if let Err(e) = diesel::insert_into(playlists::table)
        .values(&playlist)
        .on_conflict(playlists::name)
        .do_update()
        .set(&playlist)
        .execute(conn)
    {
        return Err(anyhow!(e));
    }

    let playlist_id = match playlists::table
        .filter(playlists::name.eq(playlist.name))
        .select(playlists::id)
        .first(conn)
    {
        Ok(p) => p,
        Err(NotFound) => return Ok(None),
        Err(e) => return Err(anyhow!(e)),
    };

    // Clear all current playlist items
    diesel::delete(playlists_sequences::table)
        .filter(playlists_sequences::playlist_id.eq(playlist_id))
        .execute(conn)?;

    for (i, s) in seqs.into_iter().enumerate() {
        let sequence_id = match sequences::table
            .filter(sequences::name.eq(&s.sequence))
            .select(sequences::id)
            .first(conn)
        {
            Ok(s) => s,
            Err(NotFound) => return Ok(None),
            Err(e) => return Err(anyhow!(e)),
        };

        let link = PlaylistSequence {
            playlist_id,
            sequence_id,
            sort_by: i as i32,
            enabled: s.enabled,
            play_once: s.play_once,
        };

        if let Err(e) = diesel::insert_into(playlists_sequences::table)
            .values(&link)
            .execute(conn)
        {
            return Err(anyhow!(e));
        }
    }

    Ok(Some(()))
}

pub fn del_playlist(conn: &mut SqliteConnection, playlist_name: String) -> Result<Option<()>> {
    let playlist_id = match playlists::table
        .filter(playlists::name.eq(&playlist_name))
        .select(playlists::id)
        .first::<i32>(conn)
    {
        Ok(p) => p,
        Err(NotFound) => return Ok(None),
        Err(e) => return Err(anyhow!(e)),
    };

    diesel::delete(playlists_sequences::table)
        .filter(playlists_sequences::playlist_id.eq(playlist_id))
        .execute(conn)?;

    match diesel::delete(playlists::table)
        .filter(playlists::name.eq(&playlist_name))
        .execute(conn)
    {
        Ok(_) => Ok(Some(())),
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn get_current_schedule(conn: &mut SqliteConnection) -> Result<Option<NextSchedule>> {
    let now = chrono::offset::Local::now().naive_local();

    let mut q = schedules::table.into_boxed();

    // Add today into filter
    q = match now.weekday() {
        chrono::Weekday::Mon => q.filter(schedules::monday.eq(true)),
        chrono::Weekday::Tue => q.filter(schedules::tuesday.eq(true)),
        chrono::Weekday::Wed => q.filter(schedules::wednesday.eq(true)),
        chrono::Weekday::Thu => q.filter(schedules::thursday.eq(true)),
        chrono::Weekday::Fri => q.filter(schedules::friday.eq(true)),
        chrono::Weekday::Sat => q.filter(schedules::saturday.eq(true)),
        chrono::Weekday::Sun => q.filter(schedules::sunday.eq(true)),
    };

    let schedule = match q
        .filter(schedules::enabled.eq(true))
        .filter(schedules::start_date.le(now.date().num_days_from_ce()))
        .filter(schedules::end_date.ge(now.date().num_days_from_ce()))
        .filter(schedules::start_time.le(now.time().num_seconds_from_midnight() as i64))
        .filter(schedules::end_time.ge(now.time().num_seconds_from_midnight() as i64))
        .order(schedules::start_date.asc())
        .order(schedules::start_time.asc())
        .select(Schedule::as_select())
        .first(conn)
    {
        Ok(s) => s,
        Err(NotFound) => return Ok(None),
        Err(e) => return Err(anyhow!(e)),
    };

    let playlist = match playlists::table
        .filter(playlists::id.eq(schedule.playlist_id))
        .select(Playlist::as_select())
        .first(conn)
    {
        Ok(p) => p,
        Err(NotFound) => return Ok(None),
        Err(e) => return Err(anyhow!(e)),
    };

    let sequences = match playlists_sequences::table
        .filter(playlists_sequences::playlist_id.eq(playlist.id))
        .inner_join(sequences::table)
        .select((playlists_sequences::play_once, Sequence::as_select()))
        .load::<(bool, Sequence)>(conn)
    {
        Ok(v) => v,
        Err(NotFound) => return Ok(None),
        Err(e) => return Err(anyhow!(e)),
    };

    Ok(Some((schedule, playlist, sequences)))
}

pub fn get_schedules(conn: &mut SqliteConnection) -> Result<Vec<Schedule>> {
    schedules::table
        .load::<Schedule>(conn)
        .map_err(|e| anyhow!(e))
}

pub fn get_schedule(conn: &mut SqliteConnection, schedule: String) -> Result<Option<Schedule>> {
    match schedules::table
        .filter(schedules::name.eq(schedule))
        .select(Schedule::as_select())
        .first(conn)
    {
        Ok(s) => Ok(Some(s)),
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn new_schedule(conn: &mut SqliteConnection, schedule: NewSchedule) -> Result<()> {
    match diesel::insert_into(schedules::table)
        .values(&schedule)
        .on_conflict(schedules::name)
        .do_update()
        .set(&schedule)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn del_schedule(conn: &mut SqliteConnection, schedule: String) -> Result<Option<()>> {
    match diesel::delete(schedules::table)
        .filter(schedules::name.eq(schedule))
        .execute(conn)
    {
        Ok(_) => Ok(Some(())),
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn get_scenes(conn: &mut SqliteConnection) -> Result<Vec<Scene>> {
    scenes::table.load::<Scene>(conn).map_err(|e| anyhow!(e))
}

pub fn get_scene(conn: &mut SqliteConnection, scene: String) -> Result<Option<Scene>> {
    match scenes::table
        .filter(scenes::name.eq(scene))
        .select(Scene::as_select())
        .first(conn)
    {
        Ok(s) => Ok(Some(s)),
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn new_scene(conn: &mut SqliteConnection, scene: NewScene) -> Result<()> {
    match diesel::insert_into(scenes::table)
        .values(&scene)
        .on_conflict(scenes::name)
        .do_update()
        .set(&scene)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn del_scene(conn: &mut SqliteConnection, scene: String) -> Result<Option<()>> {
    match diesel::delete(scenes::table)
        .filter(scenes::name.eq(scene))
        .execute(conn)
    {
        Ok(_) => Ok(Some(())),
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn get_meshes(conn: &mut SqliteConnection) -> Result<Vec<Mesh>> {
    meshes::table.load::<Mesh>(conn).map_err(|e| anyhow!(e))
}

pub fn get_mesh(conn: &mut SqliteConnection, mesh: String) -> Result<Option<Mesh>> {
    match meshes::table
        .filter(meshes::name.eq(mesh))
        .select(Mesh::as_select())
        .first(conn)
    {
        Ok(m) => Ok(Some(m)),
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn new_mesh(conn: &mut SqliteConnection, mesh: NewMesh) -> Result<()> {
    match diesel::insert_into(meshes::table)
        .values(&mesh)
        .on_conflict(meshes::name)
        .do_update()
        .set(&mesh)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn del_mesh(conn: &mut SqliteConnection, mesh: String) -> Result<Option<()>> {
    match diesel::delete(meshes::table)
        .filter(meshes::name.eq(mesh))
        .execute(conn)
    {
        Ok(_) => Ok(Some(())),
        Err(NotFound) => Ok(None),
        Err(e) => Err(anyhow!(e)),
    }
}
