use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

use crate::{errors::ApplicationError, models::Report};

pub async fn all_reports(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM report";
    let reports: Vec<Report> = sqlx::query_as(sql).fetch_all(&pool).await.unwrap();
    (StatusCode::OK, Json(reports))
}

pub async fn report(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Report>, ApplicationError> {
    let sql = "SELECT * FROM report WHERE id=$1";
    let report: Report = sqlx::query_as(sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ApplicationError::NotFound,
            error => error.into(),
        })?;

    Ok(Json(report))
}
