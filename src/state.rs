use std::sync::Mutex;
use crate::models::Item;

pub struct AppState {
    pub items: Mutex<Vec<Item>>,
}
