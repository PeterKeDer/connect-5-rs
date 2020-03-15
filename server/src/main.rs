mod app;
mod controllers;
mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::start()
        .await
}
