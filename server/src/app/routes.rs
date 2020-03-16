use actix_web::web;
use crate::controllers::{index, rooms};

/// Configure the app with routes.
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .service(web::scope("/rooms")
            .route("", web::get().to(rooms::get_rooms))
            .route("/create", web::post().to(rooms::post_create_room))
        );
}
