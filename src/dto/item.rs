use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
}
