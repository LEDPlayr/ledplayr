use diesel::SqliteConnection;
use tokio::sync::mpsc::Sender;

use crate::{config::Config, models::PlayerState};

pub struct State {
    pub cfg: Config,
    pub db_conn: SqliteConnection,
    pub player_ctrl: Sender<PlayerState>,
    pub player_state: PlayerState,
}
