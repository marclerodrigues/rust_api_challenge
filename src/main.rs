mod dto;
mod handlers;

use actix_web::{web, HttpResponse};
use dto::Item;
use std::sync::Mutex;

struct AppState {
    items: Mutex<Vec<Item>>,
}

async fn index(data: web::Data<AppState>) -> HttpResponse {
    let items = data.items.lock().unwrap();

    HttpResponse::Ok().json(&*items)
}

async fn create(item: web::Json<Item>, data: web::Data<AppState>) -> HttpResponse {
    println!("Received item: id={}, name={}", item.id, item.name);
    let mut items = data.items.lock().unwrap();
    let item_exists = items.iter().any(|i| i.id == item.id);

    if item_exists {
        return handlers::conflict_error_handler();
    }

    let item_clone = item.0.clone();

    items.push(item.into_inner()); // This line converts web::Json<Item> into Item

    HttpResponse::Ok().json(item_clone)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    });

    use actix_web::{App, HttpServer};

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(handlers::json_error_handler))
            .route("/items", web::post().to(create))
            .route("/items", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
