use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::request_id::MakeRequestUuid;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing_subscriber::EnvFilter;

mod controllers;
mod dto;
mod entities;
mod enums;
mod errors;
mod extractors;
mod repositories;
mod routes;
mod services;
mod utils;

struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_test_writer()
        .init();

    tracing::info!("Starting the server");

    let db_uri = std::env::var("DB_URI")?;
    let timeout_duration: u64 = std::env::var("TIMEOUT_DURATION")?.parse()?;
    let port = std::env::var("PORT")?;
    let address = format!("0.0.0.0:{port}");

    tracing::info!("Connecting to the database");
    let shared_state = Arc::new(AppState {
        db: Database::connect(db_uri).await?,
    });
    tracing::info!("Connected to the database");

    let app = Router::new()
        .merge(routes::register())
        .with_state(shared_state)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .layer(TimeoutLayer::new(Duration::from_secs(timeout_duration)))
                .compression()
                .decompression()
                .set_x_request_id(MakeRequestUuid)
                .propagate_x_request_id(),
        );

    let listener = TcpListener::bind(&address).await?;
    tracing::info!("Server started");
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Err(err) = result {
        tracing::error!("Error: {err}");
    }
}
