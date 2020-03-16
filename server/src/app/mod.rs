mod routes;

use std::sync::Mutex;
use actix_web::{web, HttpServer, HttpResponse, App, middleware::Logger};
use routes::routes;
use crate::models::AppState;

/// Start the app.
pub async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let data = web::Data::new(Mutex::new(AppState::test_data()));

    HttpServer
        ::new(move || {
            App::new()
                .app_data(data.clone())
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
