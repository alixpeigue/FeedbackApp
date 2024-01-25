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
    pub contract: i32,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct ResponseReport {
    pub id: i32,
    pub text: String,
    pub location: i32,
    pub worker: i32,
    pub contract: i32,
    pub upvoted: Option<bool>,
    pub upvotes: Option<i64>,
    pub worker_name: String,
    pub contract_description: Option<String>,
    pub location_name: String,
}
