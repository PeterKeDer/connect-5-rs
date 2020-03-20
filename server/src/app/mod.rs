mod routes;

use std::sync::Mutex;
use actix_web::{web, HttpServer, HttpResponse, HttpRequest, App, middleware::Logger, error};
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
                .app_data(web::JsonConfig::default()
                    .error_handler(handle_json_error)
                )
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

fn handle_json_error(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let err_string = err.to_string();
    error::InternalError
        ::from_response(
            err,
            HttpResponse::BadRequest().json(serde_json::json!({
                "type": "json_error",
                "error": err_string,
            })),
        )
        .into()
}
