use serde::{Serialize, Deserialize};
use connect_5_rs::Game;
use crate::models::{
    User,
    game_serde::{serialize_game, deserialize_game}
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomSettings {
    pub board_size: u32,
    pub allow_spectators: bool,
    pub public: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub id: String,
    pub settings: RoomSettings,
    pub player1: Option<User>,
    pub player2: Option<User>,
    pub spectators: Vec<User>,
    #[serde(serialize_with = "serialize_game", deserialize_with = "deserialize_game")]
    pub game: Game,
}
