use super::AppState;

use actix_web::{web, HttpResponse};
use super::dto::Item;

pub async fn index(data: web::Data<AppState>) -> HttpResponse {
    let items = data.items.lock().unwrap();

    HttpResponse::Ok().json(&*items)
}

pub async fn create(item: web::Json<Item>, data: web::Data<AppState>) -> HttpResponse {
    println!("Received item: id={}, name={}", item.id, item.name);
    let mut items = data.items.lock().unwrap();
    let item_exists = items.iter().any(|i| i.id == item.id);

    if item_exists {
        return super::handlers::conflict_error_handler();
    }

    let item_clone = item.0.clone();

    items.push(item.into_inner()); // This line converts web::Json<Item> into Item

    HttpResponse::Ok().json(item_clone)
}