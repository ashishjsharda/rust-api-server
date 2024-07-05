use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]  // Add Debug here
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
}
