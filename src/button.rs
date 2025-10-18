use core::str;
use std::{io, sync::Arc};

use futures_util::StreamExt;
use parking_lot::Mutex;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::{bytes::BytesMut, codec::Decoder, sync::CancellationToken, task::TaskTracker};

use crate::{
    config::ButtonConfig,
    db::{
        self,
        models::{Action, NewButton},
    },
    models::PlayerState,
    state::State,
};

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::other("Invalid String")),
            };
        }
        Ok(None)
    }
}

pub async fn listen(state: Arc<Mutex<State>>, cancel: CancellationToken) {
    let tracker = TaskTracker::new();

    {
        let state_locked = state.lock();

        if let Some(buttons) = &state_locked.cfg.buttons {
            for b in buttons.iter() {
                tracker.spawn(watch(cancel.clone(), state.clone(), b.clone()));
            }
        }
    }

    tracker.close();
    tracker.wait().await;
}

async fn watch(cancel: CancellationToken, state: Arc<Mutex<State>>, btn: ButtonConfig) {
    let baudrate = btn.baudrate.unwrap_or(9600);

    tracing::info!("Starting button listener on {}", btn.device);

    match tokio_serial::new(btn.device, baudrate).open_native_async() {
        Ok(port) => {
            let mut reader = LineCodec.framed(port);

            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        break;
                    },
                    Some(line_result) = reader.next() => {
                        let line = line_result.expect("Failed to read line");
                        if let Ok(button) = serde_json::from_str::<NewButton>(&line) {
                            update_button(btn.id, button, state.clone()).await;
                        }
                    }
                }
            }
        }
        Err(e) => tracing::error!("Could not open serial port: {e}"),
    }
}

pub async fn update_button(id: i32, button: NewButton, state: Arc<Mutex<State>>) {
    if button.input {
        let ctrl;
        let next_state;

        {
            let mut state = state.lock();
            ctrl = state.player_ctrl.clone();

            if let Ok(Some(btn)) = db::get_button(&mut state.db_conn, id) {
                next_state = match btn.action {
                    Action::Schedule => Some(PlayerState::Schedule),
                    Action::Playlist => Some(PlayerState::Playlist(btn.action_target)),
                    Action::Sequence => Some(PlayerState::Sequence(btn.action_target)),
                    Action::Stop => Some(PlayerState::Stop),
                    _ => None,
                };
            } else {
                next_state = None;
            }
        }

        if let Some(next_state) = next_state {
            tracing::info!("Button {id} pressed. Sending {next_state:?} to scheduler");
            if let Err(e) = ctrl.send(next_state).await {
                tracing::error!("Could not send action: {e}");
            }
        }
    }
    let mut state = state.lock();
    if let Err(e) = db::update_button(&mut state.db_conn, id, button) {
        tracing::error!("Couldn't update button: {e}");
    }
}
