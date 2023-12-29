use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Worker {
    pub id: i32,
    pub name: String,
}
