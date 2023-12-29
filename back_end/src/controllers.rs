pub mod report;

macro_rules! list_detail_view {
    ($t: ty, $n: tt) => {
        use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Json, Router};
        use sqlx::PgPool;

        use crate::errors::ApplicationError;

        pub fn router() -> Router {
            Router::new()
                .route("/", get(list))
                .route("/:id", get(detail))
        }

        async fn list(
            Extension(pool): Extension<PgPool>,
        ) -> Result<impl IntoResponse, ApplicationError> {
            let res: Vec<$t> = sqlx::query_as!($t, "SELECT * FROM " + $n)
                .fetch_all(&pool)
                .await?;
            Ok(Json(res))
        }

        async fn detail(
            Path(id): Path<i32>,
            Extension(pool): Extension<PgPool>,
        ) -> Result<impl IntoResponse, ApplicationError> {
            let contract: $t = sqlx::query_as!($t, "SELECT * FROM " + $n + " WHERE id=$1", id)
                .fetch_one(&pool)
                .await
                .map_err(|err: sqlx::Error| match err {
                    sqlx::Error::RowNotFound => ApplicationError::NotFound,
                    err => err.into(),
                })?;
            Ok(Json(contract))
        }
    };
}

pub mod contract {
    use crate::models::Contract;

    list_detail_view!(Contract, "contract");
}

pub mod client {
    use crate::models::Client;

    list_detail_view!(Client, "client");
}

pub mod worker {
    use crate::models::Worker;

    list_detail_view!(Worker, "worker");
}

pub mod location {
    use crate::models::Location;

    list_detail_view!(Location, "location");
}
