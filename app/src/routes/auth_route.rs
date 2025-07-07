use crate::AppState;
use crate::controllers::auth_controller;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth_controller::register))
        .route("/login", post(auth_controller::login))
        .route("/refresh", post(auth_controller::refresh))
}
