use axum::{
    debug_handler,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    errors::ApplicationError,
    models::{NewReport, Report},
};

#[derive(Debug, Deserialize)]
pub struct Params {
    search: Option<String>,
}

pub async fn all_reports(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, ApplicationError> {
    let reports: Vec<Report> = if let Some(search) = params.search {
        sqlx::query_as!(
            Report, 
            "SELECT id, text, worker, location, contract FROM report WHERE ts @@ to_tsquery('french', $1)",
            search
        ).fetch_all(&pool).await?
    } else {
        sqlx::query_as!(
            Report,
            "SELECT id, text, worker, location, contract FROM report"
        ).fetch_all(&pool).await?
    };
    Ok((StatusCode::OK, Json(reports)))
}

pub async fn report(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Report>, ApplicationError> {
    let report = sqlx::query_as!(
        Report,
        "SELECT id, text, worker, location, contract FROM report WHERE id=$1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => ApplicationError::NotFound,
        error => error.into(),
    })?;

    Ok(Json(report))
}

#[debug_handler]
pub async fn create_report(
    Extension(pool): Extension<PgPool>,
    Json(report): Json<NewReport>,
) -> Result<(StatusCode, Json<NewReport>), ApplicationError> {
    let sql = "INSERT INTO report (text, worker, contract, location) VALUES ($1, $2, $3, $4)";
    sqlx::query(sql)
        .bind(&report.text)
        .bind(report.worker)
        .bind(report.contract)
        .bind(report.location)
        .execute(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(report)))
}
