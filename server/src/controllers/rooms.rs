use actix_web::{HttpResponse, HttpRequest};
use serde_json::json;
use crate::models::{State, ApiResult};

pub async fn post_create_room(req: HttpRequest, state: State) -> ApiResult {
    Ok(HttpResponse::Ok().body("Create room"))
}

pub async fn get_rooms(state: State) -> ApiResult {
    let rooms = &state.lock().unwrap().rooms;

    let rooms_json = json!({
        "rooms": rooms,
    });

    Ok(HttpResponse::Ok().json(rooms_json))
}
