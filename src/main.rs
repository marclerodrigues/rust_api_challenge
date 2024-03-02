mod dto;
mod handlers;
mod items;

use actix_web::middleware::Logger;
use actix_web::{web};
use dto::Item;
use env_logger::Env;
use std::sync::Mutex;

struct AppState {
    items: Mutex<Vec<Item>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    });

    use actix_web::{App, HttpServer};

    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(handlers::json_error_handler))
            .route("/items", web::post().to(items::create))
            .route("/items", web::get().to(items::index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
