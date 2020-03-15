use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use crate::models::{AppState, JsonError};

type State = web::Data<AppState>;

type ApiResult = Result<HttpResponse, JsonError>;

pub async fn post_create_room(req: HttpRequest, state: State) -> ApiResult {
    Ok(HttpResponse::Ok().body("Create room"))
}

pub async fn get_rooms(state: State) -> ApiResult {
    let rooms = state.rooms.lock().unwrap();

    let rooms_json = json!({
        "rooms": *rooms,
    });

    Ok(HttpResponse::Ok().json(rooms_json))
}
