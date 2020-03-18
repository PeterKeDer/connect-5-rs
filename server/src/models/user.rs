use serde::{Serialize, Deserialize};

pub type UserId = uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub nickname: Option<String>,
}

impl User {
    pub fn new(nickname: Option<String>) -> User {
        User {
            id: UserId::new_v4(),
            nickname,
        }
    }
}
