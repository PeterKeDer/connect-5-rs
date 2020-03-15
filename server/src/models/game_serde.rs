use std::fmt;

use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error};
use connect_5_rs::{Game, Point, GameState, GameSide, GameStepError};

/// Serialize a game object.
///
/// ## Serialized Game
///
/// ### Fields
/// - `size`: integer representing value of `game.size()`
/// - `steps`: list of lists `[x, y]`, representing points in `game.iter_steps()`
/// - `state`: serialized state object (see below), representing value of `game.state()`
///
/// ## Serialized State
///
/// ### Fields
/// - `state`: string, either `"normal"`, `"board_full"`, or `"finished"`, depending on state
///
/// ### Optional Fields
///
/// These fields are non-null when `state` is `"finished"`, otherwise null:
/// - `points`: list with 5 lists `[x, y]`, representing `points`
/// - `side`: `0` for `GameSide::Black` or `1` for `GameSide::White`, representing `winner_side`
pub fn serialize_game<S>(game: &Game, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    GameSerializer::from_game(game).serialize(s)
}

/// Deserialize a game object.
///
/// Format is as specified in `serialize_game`.
pub fn deserialize_game<'de, D>(d: D) -> Result<Game, D::Error>
where
    D: Deserializer<'de>
{
    GameSerializer::deserialize(d)?
        .to_game()
        .map_err(Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
struct GameStateSerializer {
    state: String,
    side: Option<u16>,
    points: Option<Vec<(u32, u32)>>,
}

impl GameStateSerializer {
    fn from_state(state: &GameState) -> GameStateSerializer {
        match state {
            GameState::Normal => {
                GameStateSerializer {
                    state: String::from("normal"),
                    side: None,
                    points: None,
                }
            },
            GameState::BoardFull => {
                GameStateSerializer {
                    state: String::from("board_full"),
                    side: None,
                    points: None,
                }
            },
            GameState::Finished { winner_side, points } => {
                GameStateSerializer {
                    state: String::from("finished"),
                    side: Some(match winner_side {
                        GameSide::Black => 0,
                        GameSide::White => 1,
                    }),
                    points: Some(points.iter()
                        .map(|p| (p.x as u32, p.y as u32))
                        .collect()
                    ),
                }
            },
        }
    }

    fn to_state(&self) -> Option<GameState> {
        match &self.state[..] {
            "normal" => Some(GameState::Normal),
            "board_full" => Some(GameState::BoardFull),
            "finished" => {
                let winner_side = match self.side? {
                    0 => Some(GameSide::Black),
                    1 => Some(GameSide::White),
                    _ => None,
                }?;
                let points = self.points.as_deref()?.iter()
                    .map(|(x, y)| Point::new(*x as usize, *y as usize))
                    .collect();

                Some(GameState::Finished {
                    winner_side,
                    points,
                })
            }
            _ => None,
        }
    }
}

enum GameSerializeError {
    CannotAddStep(GameStepError),
    InvalidState,
}

impl fmt::Display for GameSerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GameSerializeError::CannotAddStep(err) => match err {
                GameStepError::InvalidPoint => "invalid point in steps - out of bounds",
                GameStepError::PointTaken => "invalid point in steps - duplicate points",
            },
            GameSerializeError::InvalidState => "invalid state - given state does not match derived state",
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSerializer {
    size: u32,
    steps: Vec<(u32, u32)>,
    state: GameStateSerializer,
}

impl GameSerializer {
    fn from_game(game: &Game) -> GameSerializer {
        let size = game.size() as u32;
        let steps = game.iter_steps()
            .map(|(_, p)| (p.x as u32, p.y as u32))
            .collect();
        let state = GameStateSerializer::from_state(game.state());

        GameSerializer {
            size,
            steps,
            state,
        }
    }

    fn to_game(&self) -> Result<Game, GameSerializeError> {
        let steps = self.steps.iter()
            .map(|(x, y)| Point::new(*x as usize, *y as usize))
            .collect::<Vec<Point>>();

        let game = match Game::from_steps(self.size as usize, &steps) {
            Ok(game) => game,
            Err(err) => return Err(GameSerializeError::CannotAddStep(err)),
        };

        // Verify that the given state is same as state derived from steps
        let given_state = match self.state.to_state() {
            Some(state) => state,
            None => return Err(GameSerializeError::InvalidState),
        };
        let derived_state = game.state();

        match (&given_state, derived_state) {
            (GameState::Normal, GameState::Normal) |
            (GameState::BoardFull, GameState::BoardFull) => (),
            // Possible to have different winning points (e.g. when connect 6 in a row)
            // However, winning side must be the same
            (
                GameState::Finished {
                    winner_side: side1,
                    points: _
                },
                GameState::Finished {
                    winner_side: side2,
                    points: _
                },
            ) if side1 == side2 => (),
            _ => return Err(GameSerializeError::InvalidState),
        };

        Ok(game)
    }
}
