use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Report {
    pub id: i32,
    pub text: String,
    pub location: i32,
    pub worker: i32,
    pub contract: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewReport {
    pub text: String,
    pub location: i32,
    pub worker: i32,
    pub contract: i32,
}
