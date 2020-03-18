use std::fmt;

use serde::{Serialize, Deserialize, Serializer, Deserializer, de::{Visitor, Error}};
use connect_5_rs::GameSide;

/// A role that a user has in a room.
#[derive(Debug)]
pub enum RoomRole {
    Player(GameSide),
    Spectator,
}

impl Serialize for RoomRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_u16(match self {
            RoomRole::Player(GameSide::Black) => 0,
            RoomRole::Player(GameSide::White) => 1,
            RoomRole::Spectator => 2,
        })
    }
}

impl<'de> Deserialize<'de> for RoomRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_u64(RoomRoleVisitor)
    }
}

struct RoomRoleVisitor;

impl<'de> Visitor<'de> for RoomRoleVisitor {
    type Value = RoomRole;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an integer between 0 and 2")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error
    {
        match v {
            0 => Ok(RoomRole::Player(GameSide::Black)),
            1 => Ok(RoomRole::Player(GameSide::White)),
            2 => Ok(RoomRole::Spectator),
            _ => Err(Error::custom("expecting integer between 0 and 2")),
        }
    }
}
