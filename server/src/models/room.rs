use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use connect_5_rs::{Game, GameSide};
use crate::models::{
    User,
    UserId,
    Result,
    Error,
    game_serde::{serialize_game, deserialize_game},
};

static MAX_ROOM_ID_LENGTH: usize = 15;
static MIN_BOARD_SIZE: usize = 9;
static MAX_BOARD_SIZE: usize = 21;
static DEFAULT_BOARD_SIZE: usize = 15;

/// The settings for a Room.
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomSettings {
    #[serde(default = "RoomSettings::default_board_size")]
    pub board_size: usize,

    #[serde(default = "RoomSettings::default_allow_spectators")]
    pub allow_spectators: bool,

    #[serde(default = "RoomSettings::default_public")]
    pub public: bool,
}

impl RoomSettings {
    fn default_board_size() -> usize {
        DEFAULT_BOARD_SIZE
    }

    fn default_allow_spectators() -> bool {
        true
    }

    fn default_public() -> bool {
        true
    }
}

impl Default for RoomSettings {
    fn default() -> Self {
        RoomSettings {
            board_size: RoomSettings::default_board_size(),
            allow_spectators: RoomSettings::default_allow_spectators(),
            public: RoomSettings::default_public(),
        }
    }
}

/// A game room where two players can play a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub id: String,
    pub settings: RoomSettings,
    pub player1: Option<User>,
    pub player2: Option<User>,
    pub spectators: HashMap<UserId, User>,
    #[serde(serialize_with = "serialize_game", deserialize_with = "deserialize_game")]
    pub game: Game,
}

impl Room {
    /// Create a new room with default values.
    fn new(id: String, settings: RoomSettings) -> Room {
        let game = Game::new(settings.board_size);
        let spectators = HashMap::new();

        Room {
            id,
            settings,
            player1: None,
            player2: None,
            spectators,
            game,
        }
    }

    /// Get the user from room with key, if it exists.
    pub fn get_user(&self, key: &RoomUserKey) -> Option<&User> {
        match key {
            RoomUserKey::Player(GameSide::Black) => self.player1.as_ref(),
            RoomUserKey::Player(GameSide::White) => self.player2.as_ref(),
            RoomUserKey::Spectator(id) => self.spectators.get(&id),
        }
    }

    /// Add user to room using a key, provided the spot isn't taken.
    pub fn add_user(&mut self, key: RoomUserKey, user: User) -> Result<()> {
        if self.get_user(&key).is_some() {
            Err(Error::bad_request("room_spot_taken"))
        } else {
            match key {
                RoomUserKey::Player(GameSide::Black) => {
                    self.player1 = Some(user);
                },
                RoomUserKey::Player(GameSide::White) => {
                    self.player2 = Some(user);
                },
                RoomUserKey::Spectator(id) => {
                    self.spectators.insert(id, user);
                },
            };
            Ok(())
        }
    }
}

// Validation
impl Room {
    fn validate_id(id: &String) -> Result<()> {
        if (1..=MAX_ROOM_ID_LENGTH).contains(&id.len()) {
            Ok(())
        } else {
            Err(Error::bad_request("invalid_room_id"))
        }
    }

    fn validate_settings(settings: &RoomSettings) -> Result<()> {
        if (MIN_BOARD_SIZE..=MAX_BOARD_SIZE).contains(&settings.board_size) {
            Ok(())
        } else {
            Err(Error::bad_request("invalid_board_size"))
        }
    }

    /// Create a room with given id and settings. If these are invalid, return the error,
    /// otherwise return the created room.
    pub fn validate(id: String, settings: RoomSettings) -> Result<Room> {
        Room::validate_id(&id)?;
        Room::validate_settings(&settings)?;

        Ok(Room::new(id, settings))
    }
}

/// A struct used to access a specific user in a given room.
#[derive(Debug, Clone, Copy)]
pub enum RoomUserKey {
    Player(GameSide),
    Spectator(UserId),
}
