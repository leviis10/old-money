use crate::AppState;
use crate::handlers::auth_handlers;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth_handlers::register))
        .route("/login", post(auth_handlers::login))
}
