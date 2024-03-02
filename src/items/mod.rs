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

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use super::*;

    use std::sync::Mutex;

    #[actix_web::test]
    async fn test_list_items_returns_a_successful_response() {
        let app_data = web::Data::new(AppState {
            items: Mutex::new(Vec::new()),
        });
        let app = test::init_service(App::new().app_data(app_data.clone()).route("/items", web::get().to(index))).await;
        let req = test::TestRequest::get().uri("/items").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }


    #[actix_web::test]
    async fn test_list_items_returns_items_in_memory() {
        let items = vec![Item { id: 1, name: "item1".to_string() }];
        let app_data = web::Data::new(AppState {
            items: Mutex::new(items),
        });

        let app = test::init_service(App::new().app_data(app_data.clone()).route("/items", web::get().to(index))).await;
        let req = test::TestRequest::get().uri("/items").to_request();
        let result = test::call_and_read_body(&app, req).await;
        assert_eq!(result, "[{\"id\":1,\"name\":\"item1\"}]");
    }


    #[actix_web::test]
    async fn test_create_item_returns_a_successful_response() {
        let app_data = web::Data::new(AppState {
            items: Mutex::new(Vec::new()),
        });
        let app = test::init_service(App::new().app_data(app_data).route("/items", web::post().to(create))).await;
        let req = test::TestRequest::post().uri("/items").set_json(&Item { id: 1, name: "item1".to_string() }).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_item_returns_the_correct_item() {
        let items = vec![Item { id: 1, name: "item1".to_string() }];
        let app_data = web::Data::new(AppState {
            items: Mutex::new(items),
        });
        let app = test::init_service(App::new().app_data(app_data).route("/items", web::post().to(create))).await;
        let req = test::TestRequest::post().uri("/items").set_json(&Item { id: 2, name: "item2".to_string() }).to_request();
        let result = test::call_and_read_body(&app, req).await;
        assert_eq!(result, "{\"id\":2,\"name\":\"item2\"}");
    }

    #[actix_web::test]
    async fn test_create_item_returns_a_conflict_when_item_exists() {
        let items = vec![Item { id: 1, name: "item1".to_string() }];
        let app_data = web::Data::new(AppState {
            items: Mutex::new(items),
        });
        let app = test::init_service(App::new().app_data(app_data).route("/items", web::post().to(create))).await;
        let req = test::TestRequest::post().uri("/items").set_json(&Item { id: 1, name: "item2".to_string() }).to_request();
        let result = test::call_service(&app, req).await;
        assert!(!result.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_item_returns_the_correct_response_when_item_exists() {
        let items = vec![Item { id: 1, name: "item1".to_string() }];
        let app_data = web::Data::new(AppState {
            items: Mutex::new(items),
        });
        let app = test::init_service(App::new().app_data(app_data).route("/items", web::post().to(create))).await;
        let req = test::TestRequest::post().uri("/items").set_json(&Item { id: 1, name: "item2".to_string() }).to_request();
        let result = test::call_service(&app, req).await;
        let body = test::read_body(result).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains(&"Item already exists".to_string()));
    }
}