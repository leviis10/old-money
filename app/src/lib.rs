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
use tracing::Level;

mod controllers;
mod dto;
mod entities;
mod enums;
mod repositories;
mod routes;
mod services;
mod utils;

struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn Error>> {
    let db_uri = std::env::var("DB_URI").unwrap();
    let timeout_duration: u64 = std::env::var("TIMEOUT_DURATION").unwrap().parse().unwrap();
    let port = std::env::var("PORT").unwrap();
    let address = format!("0.0.0.0:{port}");

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_test_writer()
        .init();

    let shared_state = Arc::new(AppState {
        db: Database::connect(db_uri).await?,
    });

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

    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub fn main() {
    let result = start();

    if let Err(err) = result {
        eprintln!("Error: {err}")
    }
}
