use std::collections::HashMap;
use connect_5_rs::{Game, Point, GameSide};
use crate::models::{StateError, Room, RoomSettings, User, UserId, RoomUserKey};

pub struct AppState {
    pub rooms: HashMap<String, Room>,
    pub users: HashMap<UserId, RoomUserKey>,
}

impl AppState {
    /// Create a new app state.
    pub fn new() -> AppState {
        AppState {
            rooms: HashMap::new(),
            users: HashMap::new(),
        }
    }

    /// Add a room to a room. If the room with same id already exist, return error.
    pub fn add_room(&mut self, room: Room) -> Result<(), StateError> {
        if self.rooms.contains_key(&room.id) {
            // TODO: move all string literals to constants file
            Err(StateError::new("duplicate_room_id"))
        } else {
            self.rooms.insert(room.id.clone(), room);
            Ok(())
        }
    }

    /// Testing data.
    pub fn test_data() -> AppState {
        let mut rooms = HashMap::new();
        let mut users = HashMap::new();
        let mut spectators = HashMap::new();

        let player2 = User::new(None);
        users.insert(player2.id, RoomUserKey::Player(GameSide::White));

        let spectator = User::new(Some(String::from("some spectator")));
        users.insert(spectator.id, RoomUserKey::Spectator(spectator.id));
        spectators.insert(spectator.id, spectator);

        rooms.insert(String::from("test id"),
            Room {
                id: String::from("test id"),
                settings: RoomSettings {
                    board_size: 15,
                    allow_spectators: true,
                    public: true,
                },
                player1: None,
                player2: Some(player2),
                game: Game::from_steps(15, &vec![
                    Point::new(0, 0),
                    Point::new(1, 2),
                    Point::new(1, 1),
                    Point::new(2, 3),
                    Point::new(2, 2),
                    Point::new(12, 2),
                    Point::new(3, 3),
                    Point::new(11, 12),
                    Point::new(4, 4), // game finishes here
                ]).unwrap(),
                spectators,
            }
        );

        AppState  {
            users,
            rooms,
        }
    }
}
