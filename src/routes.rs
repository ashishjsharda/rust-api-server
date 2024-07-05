use actix_web::web;
use crate::handlers::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::get().to(get_item_by_id))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    );
}

use crate::state::AppState;
use std::sync::Mutex;

pub fn initialize_state() -> web::Data<AppState> {
    web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    })
}
