use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_login::AuthSession;
use serde::Deserialize;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    errors::ApplicationError,
    models::{Backend, NewReport, ResponseReport},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(all_reports))
        .route("/", post(create_report))
        .route("/:id/upvotes", put(create_upvote))
        .route("/:id/upvotes", delete(delete_upvote))
        .route("/:id/invalid_votes", put(create_invalid_vote))
        .route("/:id/invalid_votes", delete(delete_invalid_vote))
        .route("/:id", get(report))
}

#[derive(Debug, Deserialize)]
pub struct Params {
    search: Option<String>,
    contract: Option<i32>,
    location: Option<i32>,
    client: Option<i32>,
}

async fn all_reports(
    Extension(pool): Extension<PgPool>,
    auth_session: AuthSession<Backend>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, ApplicationError> {
    let worker = auth_session.user.unwrap(); // should never happen
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT r.id, r.text, r.worker, r.location, r.contract, 
        EXISTS(SELECT * FROM upvote WHERE worker_id = ",
    );

    let mut list = query_builder.separated(" AND ");
    list.push_bind_unseparated(worker.id);
    list.push_unseparated(
        " and upvote.report_id = r.id) as upvoted,
        COUNT(upvote.worker_id) as upvotes, worker.name as worker_name, contract.description as contract_description, location.name as location_name
        FROM report r LEFT OUTER JOIN upvote ON upvote.report_id = r.id LEFT OUTER JOIN invalid_votes ON invalid_votes.report_id = r.id LEFT OUTER JOIN worker ON r.worker = worker.id
        LEFT OUTER JOIN contract on r.contract = contract.id LEFT OUTER JOIN location ON r.location = location.id",
    );
    if let (None, None, None, None) = (
        &params.search,
        params.contract,
        params.location,
        params.client,
    ) {
    } else {
        if let Some(_) = params.client {
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
    list.push_unseparated(" GROUP BY r.id, worker.name, contract_description, location_name HAVING COUNT(invalid_votes.*) < 2 ORDER BY upvotes DESC");
    let reports: Vec<ResponseReport> = query_builder
        .build_query_as()
        // .bind(worker.id)
        .fetch_all(&pool)
        .await?;

    Ok((StatusCode::OK, Json(reports)))
}

async fn report(
    Path(id): Path<i32>,
    auth_session: AuthSession<Backend>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<ResponseReport>, ApplicationError> {
    let worker = auth_session.user.unwrap();
    let report = sqlx::query_as!(
        ResponseReport,
        "SELECT report.id as id, text, worker, location, contract, 
        EXISTS(SELECT * FROM upvote WHERE upvote.worker_id = $1 and report_id = report.id) as upvoted,
        COUNT(upvote.worker_id) as upvotes, worker.name as worker_name, contract.description as contract_description, location.name as location_name
        FROM report LEFT OUTER JOIN upvote ON upvote.report_id = report.id LEFT OUTER JOIN invalid_votes ON invalid_votes.report_id = report.id LEFT OUTER JOIN worker ON report.worker = worker.id
        LEFT OUTER JOIN contract on report.contract = contract.id LEFT OUTER JOIN location ON report.location = location.id
        WHERE report.id=$2
        GROUP BY report.id, worker_name, contract_description, location_name
        HAVING COUNT(invalid_votes.*) < 2",
        worker.id,
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

async fn create_report(
    Extension(pool): Extension<PgPool>,
    auth_session: AuthSession<Backend>,
    Json(report): Json<NewReport>,
) -> Result<(StatusCode, Json<NewReport>), ApplicationError> {
    let worker = auth_session.user.unwrap(); // should never panic
    let sql = "INSERT INTO report (text, worker, contract, location) VALUES ($1, $2, $3, $4)";
    sqlx::query(sql)
        .bind(&report.text)
        .bind(worker.id)
        .bind(report.contract)
        .bind(report.location)
        .execute(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(report)))
}

async fn create_upvote(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    auth_session: AuthSession<Backend>,
) -> Result<StatusCode, ApplicationError> {
    let worker = auth_session.user.unwrap(); // should never happen
    sqlx::query!(
        "INSERT INTO upvote (worker_id, report_id) VALUES ($1, $2)",
        worker.id,
        id
    )
    .execute(&pool)
    .await?;
    Ok(StatusCode::CREATED)
}

async fn delete_upvote(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    auth_session: AuthSession<Backend>,
) -> Result<impl IntoResponse, ApplicationError> {
    let worker = auth_session.user.unwrap(); // should never happen
    sqlx::query!(
        "DELETE FROM upvote WHERE worker_id = $1 AND report_id = $2",
        worker.id,
        id
    )
    .execute(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => ApplicationError::NotFound,
        err => err.into(),
    })?;
    Ok(())
}

async fn create_invalid_vote(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    auth_session: AuthSession<Backend>,
) -> Result<StatusCode, ApplicationError> {
    let worker = auth_session.user.unwrap();
    sqlx::query!(
        "INSERT INTO invalid_votes (worker_id, report_id) VALUES ($1, $2)",
        worker.id,
        id
    )
    .execute(&pool)
    .await?;
    Ok(StatusCode::CREATED)
}

async fn delete_invalid_vote(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    auth_session: AuthSession<Backend>,
) -> Result<impl IntoResponse, ApplicationError> {
    let worker = auth_session.user.unwrap();
    sqlx::query!(
        "DELETE FROM invalid_votes WHERE worker_id = $1 AND report_id = $2",
        worker.id,
        id
    )
    .execute(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => ApplicationError::NotFound,
        err => err.into(),
    })?;
    Ok(())
}
