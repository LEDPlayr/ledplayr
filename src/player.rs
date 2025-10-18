use core::time;
use std::{cmp::Ordering, collections::HashMap, net::Ipv4Addr, sync::Arc};

use anyhow::{anyhow, Context, Result};
use chrono::NaiveTime;
use ddp_rs::{connection, protocol};
use parking_lot::Mutex;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    db::{self, models::NextSchedule},
    models::{PlayerState, PlayerStatus},
    patterns,
    state::State,
    storage,
};

struct Data {
    offset: usize,
    data: Vec<u8>,
}

struct SenderConfig {
    offset: usize,
    len: usize,
    chan: Sender<Data>,
}

pub async fn controller(
    cancel: CancellationToken,
    mut player_ctrl: Receiver<PlayerState>,
    next_state: Sender<PlayerState>,
    auto_start: bool,
) {
    if auto_start {
        if let Err(e) = next_state.send(PlayerState::Schedule).await {
            tracing::error!("Could not auto start scheduler: {e}");
        }
    }

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                return;
            },
            s = player_ctrl.recv() => {
                if let Some(s) = s {
                    if s != PlayerState::Stop {
                        if let Err(e) = next_state.send(PlayerState::Stop).await {
                            tracing::error!("Could not stop scheduler: {e}");
                        }
                    }
                    if let Err(e) = next_state.send(s).await {
                        tracing::error!("Could not start scheduler: {e}");
                    }
                }
            }
        }
    }
}

pub async fn start_scheduler(
    state: Arc<Mutex<State>>,
    cancel: CancellationToken,
    mut next_state: Receiver<PlayerState>,
) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                return;
            },
            s = next_state.recv() => {
                if let Some(s) = s {
                    match s {
                        PlayerState::Schedule => {
                            let cancel = cancel.child_token();
                            scheduler(state.clone(), cancel.clone(), &mut next_state).await;
                        },
                        PlayerState::Test(tests) => {
                            let cancel = cancel.child_token();
                            tester(state.clone(), cancel.clone(), &mut next_state, tests).await;
                        },
                        _ => {}
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

    {
        let mut state = state.lock();
        state.player_status = PlayerStatus::Scheduler;
    }

    let tracker = TaskTracker::new();
    let s = match start_senders(state.clone(), &tracker).await {
        Ok(s) => Some(s),
        Err(e) => {
            tracing::error!("{e}");
            None
        }
    };
    tracker.close();

    if let Some(s) = s {
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
                _ = interval.tick() => check_for_schedules(
                    state.clone(),
                    cancel.clone(),
                    s.clone(),
                    player_state
                ).await,
            }
        }

        drop(s);
        tracker.wait().await;
    }

    {
        let mut state = state.lock();
        state.player_status = PlayerStatus::Stopped;
    }

    tracing::info!("Scheduler thread stopped");
}

async fn check_for_schedules(
    state: Arc<Mutex<State>>,
    cancel: CancellationToken,
    s: Sender<Data>,
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
    s: Sender<Data>,
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
            "Playlist loop: {loop_count}, sequence: {}({seq_idx}){} - frames: {}@{}ms",
            sequence.name,
            match play_once {
                true => "[once]",
                false => "[repeat]",
            },
            sequence.frames,
            sequence.step_time,
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
                                s.send(Data{offset:0, data:f}).await.context("Couldn't send frame")?;
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

async fn start_senders(state: Arc<Mutex<State>>, tracker: &TaskTracker) -> Result<Sender<Data>> {
    // Load controllers
    let mut controllers = Vec::new();

    // Don't lock forever
    {
        let state = state.lock();

        match storage::read_outputs(&state.cfg) {
            Ok(channels) => {
                for c in channels.channel_outputs.iter() {
                    for u in c.universes.iter() {
                        controllers.push((
                            u.address,
                            u.start_channel as usize,
                            u.channel_count as usize,
                        ));
                    }
                }
            }
            Err(e) => {
                return Err(anyhow!("Could not start senders: {e}"));
            }
        };
    }

    let mut senders = Vec::new();

    let mut port = 4048;
    for (ip, start, len) in controllers.iter() {
        let (data_out, sender_rx) = mpsc::channel::<Data>(1);
        tracker.spawn(sender(*ip, port, *len, sender_rx));
        senders.push(SenderConfig {
            offset: *start - 1,
            len: *len,
            chan: data_out,
        });
        port += 1;
    }

    senders.sort_by(|a, b| a.offset.cmp(&b.offset));

    // Spawn the demuxer
    let (s, r) = mpsc::channel::<Data>(1);
    tracker.spawn(demuxer(r, senders));

    Ok(s)
}

async fn demuxer(mut data_in: Receiver<Data>, senders: Vec<SenderConfig>) {
    tracing::info!("Started demuxer for {} controllers", senders.len());

    while let Some(data) = data_in.recv().await {
        let mut d_start = data.offset;
        let mut data = data.data.as_slice();

        for cfg in senders.iter() {
            let d_end = d_start + data.len() - 1;
            let s_start = cfg.offset;
            let s_end = s_start + cfg.len - 1;

            if d_start >= s_start && d_start < s_end {
                let offset = d_start - s_start;

                if d_end <= s_end {
                    cfg.chan
                        .send(Data {
                            offset,
                            data: data.to_vec(),
                        })
                        .await
                        .unwrap();
                    break;
                } else {
                    let spl = data.split_at(s_end - d_start + 1);

                    data = spl.1;

                    cfg.chan
                        .send(Data {
                            offset,
                            data: spl.0.to_vec(),
                        })
                        .await
                        .unwrap();
                    d_start = s_end + 1;
                }
            }
        }
    }

    tracing::info!("Stopped demuxer for {} controllers", senders.len());
}

async fn sender(ip: Ipv4Addr, port: u16, len: usize, mut r: Receiver<Data>) {
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

    while let Some(mut data) = r.recv().await {
        let d_len = data.data.len();
        let d_end = data.offset + d_len;

        let to_send = match d_end.cmp(&len) {
            Ordering::Equal => {
                if data.offset == 0 {
                    // Everything is ok
                    Ok(data.data)
                } else {
                    // Pad start
                    let mut d = vec![0u8; data.offset];
                    d.append(&mut data.data);
                    Ok(d)
                }
            }
            Ordering::Less => {
                // Pad end
                let mut pad = vec![0u8; d_end - d_len];
                data.data.append(&mut pad);
                Ok(data.data)
            }
            Ordering::Greater => Err(anyhow!("Too much data for sender {}>{}", d_len, len)),
        };

        match to_send {
            Ok(data) => {
                conn.write(&data).unwrap();
            }
            Err(e) => tracing::warn!("{e}"),
        }
    }

    tracing::info!("Stopped sender for controller: {ip}");
}

async fn tester(
    state: Arc<Mutex<State>>,
    cancel: CancellationToken,
    player_state: &mut Receiver<PlayerState>,
    tests: patterns::TestSpec,
) {
    tracing::info!("Testing thread started");

    let mut model_lookup = HashMap::new();

    // Don't lock forever
    {
        let mut state = state.lock();

        match storage::read_models(&state.cfg) {
            Ok(models) => {
                for m in models.into_iter() {
                    model_lookup.insert(m.name.clone(), m);
                }
            }
            Err(e) => {
                tracing::error!("Could not start tester: {e}");
                return;
            }
        };

        state.player_status = PlayerStatus::Testing;
    }

    // (start, len, sequence)
    let mut test_setup = Vec::new();
    for (model, sequence) in tests.tests.iter() {
        if let Some(m) = model_lookup.get(model) {
            if m.channel_count % 3 != 0 {
                tracing::error!("Can't handle a non multiple of 3 channel count");
                cancel.cancel();
                return;
            }

            let start = (m.start_channel - 1) / 3;
            let len = m.channel_count / 3;

            test_setup.push((start as usize, len as usize, sequence));
        } else {
            tracing::error!("Invalid model specified '{model}'");
            cancel.cancel();
            return;
        }
    }
    test_setup.sort_by(|a, b| a.0.cmp(&b.0));

    let mut intvl = tokio::time::interval(time::Duration::from_millis(tests.step_ms));
    intvl.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let tracker = TaskTracker::new();
    let s = match start_senders(state.clone(), &tracker).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("{e}");
            return;
        }
    };
    tracker.close();

    let mut loop_count = 0;
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
            _ = intvl.tick() => {
                let mut to_send = Vec::new();
                let mut start_channel: Option<usize> = None;
                let mut last_channel = 0;

                for (start, len, seq) in test_setup.iter() {
                    if *start != last_channel && !to_send.is_empty() {
                        if let Some(start_channel) = start_channel {
                            s.send(Data{offset: start_channel * 3, data: to_send})
                                    .await
                                    .context("Couldn't send frame")
                                    .unwrap();
                            to_send = Vec::new();
                        }
                        start_channel = None;
                    }

                    if start_channel.is_none() {
                        start_channel = Some(*start);
                    }

                    let mut data = seq.as_vec(*len);
                    if seq.moves() {
                        data.rotate_right((loop_count % len) * 3);
                    }
                    to_send.append(&mut data);
                    last_channel = start + len;
                }

                if let Some(start_channel) = start_channel {
                    s.send(Data{offset: start_channel * 3, data: to_send})
                            .await
                            .context("Couldn't send frame")
                            .unwrap();
                }

                loop_count += 1
            }
        }
    }

    drop(s);
    tracker.wait().await;

    {
        let mut state = state.lock();
        state.player_status = PlayerStatus::Stopped;
    }

    tracing::info!("Testing thread stopped");
}
