use axum::{
    debug_handler,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    errors::ApplicationError,
    models::{NewReport, Report},
};

#[derive(Debug, Deserialize)]
pub struct Params {
    search: Option<String>,
    contract: Option<i32>,
    location: Option<i32>,
    client: Option<i32>,
}

pub async fn all_reports(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, ApplicationError> {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT r.id, r.text, r.worker, r.location, r.contract FROM report r");
    let mut list = query_builder.separated(" AND ");
    if let (None, None, None, None) = (
        &params.search,
        params.contract,
        params.location,
        params.client,
    ) {
    } else {
        if let Some(client) = params.client {
            list.push_unseparated(" JOIN contract co ON co.id = r.contract");
        }
        list.push_unseparated(" WHERE ");
        if let Some(search) = params.search {
            list.push_unseparated("ts @@ to_tsquery('french', ");
            list.push_bind_unseparated(search);
            list.push(" )");
        }
        if let Some(contract) = params.contract {
            list.push("r.contract = ");
            list.push_bind_unseparated(contract);
        }
        if let Some(location) = params.location {
            list.push("r.location = ");
            list.push_bind_unseparated(location);
        }
        if let Some(client) = params.client {
            list.push("co.client = ");
            list.push_bind_unseparated(client);
        }
    }
    let reports: Vec<Report> = query_builder.build_query_as().fetch_all(&pool).await?;

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
