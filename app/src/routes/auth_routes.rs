use crate::AppState;
use crate::controllers::auth_controllers;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth_controllers::register))
        .route("/login", post(auth_controllers::login))
}
