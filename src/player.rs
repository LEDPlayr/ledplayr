use core::time;
use std::{net::Ipv4Addr, sync::Arc};

use anyhow::{Context, Result};
use chrono::NaiveTime;
use ddp_rs::{connection, protocol};
use parking_lot::Mutex;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    db::{self, models::NextSchedule},
    models::PlayerState,
    state::State,
    storage,
};

pub async fn start_scheduler(
    state: Arc<Mutex<State>>,
    cancel: CancellationToken,
    mut player_state: Receiver<PlayerState>,
) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                return;
            },
            s = player_state.recv() => {
                if let Some(s) = s {
                    if s == PlayerState::Start {
                        let cancel = cancel.child_token();
                        scheduler(state.clone(), cancel.clone(), &mut player_state).await;
                    }
                }
            }
        }
    }
}

async fn scheduler(
    state: Arc<Mutex<State>>,
    cancel: CancellationToken,
    player_state: &mut Receiver<PlayerState>,
) {
    tracing::info!("Scheduler thread started");

    let tracker = TaskTracker::new();

    // Load controllers
    let mut controllers = Vec::new();

    // Don't lock forever
    {
        let mut state = state.lock();

        match storage::read_outputs(&state.cfg) {
            Ok(channels) => {
                for c in channels.channel_outputs.iter() {
                    for u in c.universes.iter() {
                        controllers.push((u.address, u.channel_count as usize));
                    }
                }
            }
            Err(e) => {
                tracing::error!("Could not start scheduler: {e}");
                return;
            }
        };

        state.player_state = PlayerState::Start;
    }

    let mut senders = Vec::new();

    let mut port = 4048;
    for (ip, channels) in controllers.iter() {
        let (data_out, sender_rx) = mpsc::channel::<Vec<u8>>(1);
        tracker.spawn(sender(*ip, port, sender_rx));
        senders.push((data_out, *channels));
        port += 1;
    }

    // Spawn the demuxer
    let (s, r) = mpsc::channel::<Vec<u8>>(1);
    tracker.spawn(demuxer(r, senders));

    tracker.close();

    let mut interval = tokio::time::interval(time::Duration::from_secs(10));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            s = player_state.recv() => {
                if let Some(s) = s {
                    if s == PlayerState::Stop {
                        cancel.cancel();
                        break;
                    }
                }
            }
            _ = interval.tick() => check_for_schedules(state.clone(), cancel.clone(), s.clone(), player_state).await,
        }
    }

    drop(s);
    tracker.wait().await;

    {
        let mut state = state.lock();
        state.player_state = PlayerState::Stop;
    }

    tracing::info!("Scheduler thread stopped");
}

async fn check_for_schedules(
    state: Arc<Mutex<State>>,
    cancel: CancellationToken,
    s: Sender<Vec<u8>>,
    player_state: &mut Receiver<PlayerState>,
) {
    tracing::debug!("Checking for schedules");

    let mut next = None;

    {
        let mut state = state.lock();
        match db::get_current_schedule(&mut state.db_conn) {
            Ok(Some(s)) => {
                tracing::debug!("Schedule found: {}", s.0.name);
                next = Some(s);
            }
            Ok(None) => {
                tracing::debug!("No upcoming schedule");
            }
            Err(e) => {
                tracing::error!("Error checking schedule: {e}");
            }
        }
    }

    if let Some(next) = next {
        if let Err(e) = play_schedule(state, next, cancel, s, player_state).await {
            tracing::error!("Error playing schedule: {e}");
        }
    }
}

async fn play_schedule(
    state: Arc<Mutex<State>>,
    next: NextSchedule,
    cancel: CancellationToken,
    s: Sender<Vec<u8>>,
    player_state: &mut Receiver<PlayerState>,
) -> Result<()> {
    let (schedule, playlist, sequences) = next;

    // Create an interval per framerate
    let mut intervals = Vec::new();
    for s in sequences.iter() {
        intervals.push(tokio::time::interval(time::Duration::from_millis(
            s.1.step_time as u64,
        )))
    }

    // Set up an Interval to fire at the end of the schedule
    let end_time = NaiveTime::from_num_seconds_from_midnight_opt(schedule.end_time as u32, 0)
        .context("Cannot set end_time")?;
    let end_time = chrono::Local::now()
        .with_time(end_time)
        .single()
        .context("Cannot set end_time")?;
    let remaining = end_time - chrono::Local::now();
    let end = tokio::time::Instant::now()
        + tokio::time::Duration::from_secs(remaining.num_seconds() as u64);
    let mut end = tokio::time::interval_at(end, time::Duration::from_millis(10));

    tracing::info!("Starting playist {}, ending at {}", playlist.name, end_time);

    let mut loop_count = 0;
    let mut seq_idx = 0;
    let mut seq = None;

    while !cancel.is_cancelled() && (playlist.repeat || loop_count < playlist.loop_count) {
        let (play_once, sequence) = sequences.get(seq_idx).context("Couldn't get sequence")?;
        tracing::info!(
            "Playlist loop: {loop_count}, sequence: {}({seq_idx}) - frames: {}@{}ms",
            sequence.name,
            sequence.frames,
            sequence.step_time
        );

        if seq.is_none() {
            tracing::info!("Loading sequence: {}", sequence.name);
            let state = state.lock();
            seq = storage::read_sequence_meta(&state.cfg, &sequence.name)
                .context("Couldn't read sequence meta")?;
        }

        let mut frame = 0;
        let int = intervals
            .get_mut(seq_idx)
            .context("Couldn't get interval")?;

        while frame < sequence.frames {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(()),
                _ = end.tick() => return Ok(()),
                s = player_state.recv() => {
                    if let Some(s) = s {
                        if s == PlayerState::Stop {
                            cancel.cancel();
                            return Ok(());
                        }
                    }
                },
                _ = int.tick() => {
                    if let Some(ref mut seq) = seq {
                        match seq.get_frame(frame as u32) {
                            Ok(Some(f)) => {
                                s.send(f).await.context("Couldn't send frame")?;
                            },
                            Ok(None) => break,
                            Err(e) => {
                                tracing::error!("Error reading frame: {e}");
                                break;
                            }
                        }
                    }
                    frame += 1;
                }
            }
        }

        if *play_once {
            seq_idx += 1;

            if seq_idx >= sequences.len() {
                loop_count += 1;
                seq_idx = 0;
            }

            seq = None;
        }
    }

    Ok(())
}

async fn demuxer(mut data_in: Receiver<Vec<u8>>, senders: Vec<(Sender<Vec<u8>>, usize)>) {
    tracing::info!("Started demuxer for {} controllers", senders.len());

    while let Some(data) = data_in.recv().await {
        let mut data = data.as_slice();

        for (s, channels) in senders.iter() {
            if data.len() > *channels {
                let spl = data.split_at(*channels);
                data = spl.1;

                s.send(spl.0.to_vec()).await.unwrap();
            } else {
                tracing::warn!("Not enough data to send to output");
            }
        }
    }

    tracing::info!("Stopped demuxer for {} controllers", senders.len());
}

async fn sender(ip: Ipv4Addr, port: u16, mut r: Receiver<Vec<u8>>) {
    let mut conn = connection::DDPConnection::try_new(
        format!("{ip}:4048"),
        protocol::PixelConfig::default(),
        protocol::ID::Default,
        std::net::UdpSocket::bind(format!("0.0.0.0:{port}"))
            .context("Failed to start UDP listener")
            .unwrap(),
    )
    .context("Failed to create DDP connection")
    .unwrap();

    tracing::info!("Started sender for controller: {ip}");

    while let Some(data) = r.recv().await {
        conn.write(&data).unwrap();
    }

    tracing::info!("Stopped sender for controller: {ip}");
}
