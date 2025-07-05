use crate::AppState;
use crate::controllers::roles_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(roles_controller::get_all))
        .route("/{id}", get(roles_controller::get_by_id))
        .route("/", post(roles_controller::create))
        .route("/{id}", put(roles_controller::update_by_id))
        .route("/{id}", delete(roles_controller::delete_by_id))
        .fallback(roles_controller::not_found)
}
