use std::sync::Mutex;
use crate::models::Room;

pub struct AppState {
    pub counter: Mutex<i32>,
    pub rooms: Mutex<Vec<Room>>,
}
