use actix_web::{HttpResponse, web};
use serde_json::json;
use uuid::Uuid;

use crate::model::Item;

pub async fn create_item(item: web::Json<Item>) -> HttpResponse {
    let created_item = item.into_inner();
    HttpResponse::Created().json(json!({
        "message": "Item created successfully",
        "endpoint": "POST /items",
        "data": created_item
    }))
}

pub async fn get_items() -> HttpResponse {
    let items = vec![
        Item {
            id: Uuid::new_v4(),
            name: String::from("Sample Item 1"),
            description: String::from("This is a sample item for testing the GET /items endpoint"),
        },
        Item {
            id: Uuid::new_v4(),
            name: String::from("Sample Item 2"),
            description: String::from("Another sample item to demonstrate the API response"),
        },
    ];
    HttpResponse::Ok().json(json!({
        "message": "Items retrieved successfully",
        "endpoint": "GET /items",
        "count": items.len(),
        "data": items
    }))
}

pub async fn get_item(item_id: web::Path<Uuid>) -> HttpResponse {
    let item = Item {
        id: *item_id,
        name: String::from("Sample Item"),
        description: String::from("This is a sample item retrieved by ID"),
    };
    HttpResponse::Ok().json(json!({
        "message": "Item retrieved successfully",
        "endpoint": format!("GET /items/{}", item_id),
        "data": item
    }))
}

pub async fn update_item(item_id: web::Path<Uuid>, item: web::Json<Item>) -> HttpResponse {
    let mut updated_item = item.into_inner();
    updated_item.id = *item_id;
    HttpResponse::Ok().json(json!({
        "message": "Item updated successfully",
        "endpoint": format!("PUT /items/{}", item_id),
        "data": updated_item
    }))
}

pub async fn delete_item(item_id: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "Item deleted successfully",
        "endpoint": format!("DELETE /items/{}", item_id),
        "deleted_item_id": item_id.to_string()
    }))
}
