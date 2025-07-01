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

mod dto;
mod entities;
mod handlers;
mod routes;
mod utils;

struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_test_writer()
        .init();

    let shared_state = Arc::new(AppState {
        db: Database::connect("postgres://postgres:password@localhost:5432/old_money").await?,
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
                .layer(TimeoutLayer::new(Duration::from_secs(60)))
                .compression()
                .decompression()
                .set_x_request_id(MakeRequestUuid)
                .propagate_x_request_id(),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub fn main() {
    let result = start();

    if let Err(err) = result {
        eprintln!("Error: {err}")
    }
}
