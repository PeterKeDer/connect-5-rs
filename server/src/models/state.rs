use connect_5_rs::{Game, Point};
use crate::models::{Room, RoomSettings, User};

pub struct AppState {
    pub rooms: Vec<Room>,
}

impl AppState {
    /// Create a new app state.
    pub fn new() -> AppState {
        AppState {
            rooms: vec![],
        }
    }

    /// Testing data.
    pub fn test_data() -> AppState {
        AppState  {
            rooms: vec![
                Room {
                    id: String::from("test id"),
                    settings: RoomSettings {
                        board_size: 15,
                        allow_spectators: true,
                        public: true,
                    },
                    player1: None,
                    player2: Some(User {
                        id: uuid::Uuid::new_v4(),
                        nickname: String::from("test nickname"),
                    }),
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
                    spectators: vec![
                        User {
                            id: uuid::Uuid::new_v4(),
                            nickname: String::from("some spectator"),
                        },
                    ],
                }
            ],
        }
    }
}
