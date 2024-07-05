use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;

use crate::models::Item;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct ItemInput {
    pub name: String,
    pub price: f64,
}

pub async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    println!("Fetching all items. Current items: {:?}", *items);  // This line requires Item to derive Debug
    HttpResponse::Ok().json(&*items)
}

pub async fn get_item_by_id(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    if let Some(item) = items.iter().find(|&item| item.id == *path) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn create_item(item: web::Json<ItemInput>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let new_item = Item {
        id: Uuid::new_v4(),
        name: item.name.clone(),
        price: item.price,
    };
    items.push(new_item);
    println!("Item created. Current items: {:?}", *items);  // This line requires Item to derive Debug
    HttpResponse::Created().finish()
}

pub async fn update_item(path: web::Path<Uuid>, item: web::Json<ItemInput>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let mut updated_item = None;

    if let Some(existing_item) = items.iter_mut().find(|item| item.id == *path) {
        existing_item.name = item.name.clone();
        existing_item.price = item.price;
        updated_item = Some(existing_item.clone());
    }

    if let Some(updated_item) = updated_item {
        println!("Item updated. Current items: {:?}", *items);  // This line requires Item to derive Debug
        HttpResponse::Ok().json(updated_item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn delete_item(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let mut deleted = false;

    if let Some(pos) = items.iter().position(|item| item.id == *path) {
        items.remove(pos);
        deleted = true;
    }

    if deleted {
        println!("Item deleted. Current items: {:?}", *items);  // This line requires Item to derive Debug
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
