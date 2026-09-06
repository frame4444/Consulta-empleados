use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub salary: f64,
}
#[derive(Debug, Deserialize)]
pub struct NewEmployee {
    pub name: String,
    pub salary: f64,
}

#[derive(Debug, Serialize)]
pub struct InsertResult {
    pub success: bool,
    pub message: String,
}
