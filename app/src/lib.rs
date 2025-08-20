use crate::constants::environment_constants::{DB_URI, PORT, TIMEOUT_DURATION};
use crate::docs::ApiDoc;
use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::signal::unix::SignalKind;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::request_id::MakeRequestUuid;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa::openapi::ServerBuilder;
use utoipa_swagger_ui::SwaggerUi;

mod constants;
mod controllers;
mod docs;
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

    let db_uri = std::env::var(DB_URI)?;
    let timeout_duration: u64 = std::env::var(TIMEOUT_DURATION)?.parse()?;
    let port = std::env::var(PORT)?;
    let address = format!("0.0.0.0:{port}");

    tracing::info!("Connecting to the database");
    let shared_state = Arc::new(AppState {
        db: Database::connect(db_uri).await?,
    });
    tracing::info!("Connected to the database");

    let mut app = Router::new()
        .merge(routes::register())
        .with_state(Arc::clone(&shared_state))
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

    if cfg!(debug_assertions) {
        let mut openapi = ApiDoc::openapi();
        openapi.info.title = String::from("old-money");
        openapi.servers = Some(vec![
            ServerBuilder::new()
                .url(format!("http://localhost:{port}"))
                .description(Some("Local Development Server"))
                .build(),
        ]);
        app = app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi));
    }

    let listener = TcpListener::bind(&address).await?;
    tracing::info!("Server started on port {port}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(Arc::clone(&shared_state)))
        .await?;

    Ok(())
}

async fn shutdown_signal(state: Arc<AppState>) {
    let ctrl_c = async { signal::ctrl_c().await.unwrap() };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(SignalKind::terminate())
            .unwrap()
            .recv()
            .await
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }

    state.db.to_owned().close().await.unwrap();
}

pub fn main() {
    let result = start();

    if let Err(err) = result {
        tracing::error!("Error: {err}");
    }
}
