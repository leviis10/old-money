use crate::AppState;
use crate::controllers::users_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(users_controller::get_all))
        .route("/{id}", get(users_controller::get_by_id))
        .route("/", post(users_controller::create))
        .route("/{id}", put(users_controller::update_by_id))
        .route("/{id}", delete(users_controller::delete_by_id))
        .fallback(users_controller::not_found)
}
