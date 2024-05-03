use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftGame {
    pub id: u64,
    pub game_code: String,
    pub game_name: String,
    pub players: Vec<u64>,
    pub private: bool,
    pub teams: bool,
    pub computer_filled: bool,
}