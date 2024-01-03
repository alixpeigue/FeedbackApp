use axum_login::{login_required, AuthManagerLayerBuilder};
use std::fs;
use time::Duration;

use anyhow::Context;
use axum::{routing::get, Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tower_sessions::{ExpiredDeletion, Expiry, PostgresStore, SessionManagerLayer};
use tracing_subscriber::prelude::*;

use crate::models::Backend;

mod auth;
mod controllers;
mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    assert_eq!(key, "DATABASE_URL");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
        .context("can't connect to database")?;

    // Session layer.
    //
    // This uses `tower-sessions` to establish a layer that will provide the session
    // as a request extension.
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let _deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = Backend::new(pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .nest("/reports", controllers::report::router())
        .nest("/contracts", controllers::contract::router())
        .nest("/clients", controllers::client::router())
        .nest("/workers", controllers::worker::router())
        .nest("/locations", controllers::location::router())
        .route_layer(login_required!(Backend))
        .route("/", get(root))
        .merge(auth::router())
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(auth_layer);

    // run it with hyper
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
