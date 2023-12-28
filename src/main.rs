use std::time::Duration;

use anyhow::Context;
use axum::{routing::get, Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod controllers;
mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    tracing_subscriber::fmt::init();

    let db_connection_str = "postgres://postgres:passwd@localhost".to_string();

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .context("can't connect to database")?;

    let app = Router::new()
        .route("/", get(root))
        .route("/reports", get(controllers::report::all_reports))
        .route("/report/:id", get(controllers::report::report))
        .layer(Extension(pool));

    // run it with hyper
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
