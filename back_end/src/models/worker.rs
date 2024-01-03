use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Worker {
    pub id: i32,
    pub name: String,
}

impl AuthUser for Worker {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.name.as_bytes()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: PgPool,
}

impl Backend {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = Worker;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as!(
            Self::User,
            "SELECT * FROM worker WHERE name = $1",
            &creds.name
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> =
            sqlx::query_as!(Self::User, "SELECT * FROM worker WHERE id = $1", user_id)
                .fetch_optional(&self.db)
                .await?;

        Ok(user)
    }
}
