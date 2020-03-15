pub mod rooms;

use actix_web::{web, HttpRequest};
use crate::models::AppState;

pub async fn index() -> &'static str {
    "Hello world!"
}

pub async fn count(req: HttpRequest, data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    println!("REQ: {:?}", req);

    format!("Counter: {}", *counter)
}
