mod routes;

use std::sync::Mutex;
use actix_web::{web, HttpServer, HttpResponse, App, middleware::Logger};
use routes::routes;
use crate::models::{AppState, Room, RoomSettings, User};

use connect_5_rs::{Game, Point};

/// Start the app.
pub async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let state = web::Data::new(AppState {
        counter: Mutex::new(0),
        // Testing data
        rooms: Mutex::new(vec![
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
        ]),
    });

    HttpServer
        ::new(move || {
            App::new()
                .app_data(state.clone())
                .wrap(Logger::default())
                .configure(routes)
                .default_service(web::resource("")
                    .route(web::get().to(HttpResponse::NotFound))
                    .route(web::route().to(HttpResponse::MethodNotAllowed))
                )
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
