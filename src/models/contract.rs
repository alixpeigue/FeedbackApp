use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Contract {
    pub id: i32,
    pub description: String,
    pub client: i32,
}
