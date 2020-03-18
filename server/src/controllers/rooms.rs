use actix_web::{HttpResponse, HttpRequest, web::{self, Json}};
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::models::{
    AppData,
    ApiResult,
    Room,
    User,
    UserId,
    Error,
    RoomSettings,
    RoomRole,
    RoomUserKey,
};

#[derive(Serialize, Deserialize)]
pub struct PostCreateRoomParams {
    room_id: String,
    nickname: Option<String>,
    role: RoomRole,
    settings: Option<RoomSettings>,
}

#[derive(Serialize)]
struct PostCreateRoomResponse {
    user_id: UserId,
    room_id: String,
}

pub async fn post_create_room(
    params: Json<PostCreateRoomParams>,
    data: AppData,
) -> ApiResult {
    let mut state = data.lock().map_err(|_| Error::internal())?;
    let params = params.into_inner();

    println!("{}", serde_json::to_string_pretty(&params).unwrap());

    // Create room if params are valid
    let mut room = Room::validate(params.room_id, params.settings.unwrap_or_default())?;
    let room_id = room.id.clone();

    // Create user, and add it to the room
    let user = User::new(params.nickname);
    let user_id = user.id;

    let key = match params.role {
        RoomRole::Player(side) => RoomUserKey::Player(side),
        RoomRole::Spectator => RoomUserKey::Spectator(user_id),
    };
    room.add_user(key, user).unwrap();

    // Try adding room, potential duplicate id
    state.add_room(room)?;

    Ok(HttpResponse::Ok().json(PostCreateRoomResponse {
        user_id,
        room_id,
    }))
}

pub async fn get_rooms(data: AppData) -> ApiResult {
    let rooms = &data.lock()
        .map_err(|_| Error::internal())?
        .rooms;

    let rooms_json = json!({
        "rooms": rooms,
    });

    Ok(HttpResponse::Ok().json(rooms_json))
}
