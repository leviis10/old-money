use crate::AppState;
use crate::controllers::roles_controllers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(roles_controllers::get_all))
        .route("/{id}", get(roles_controllers::get_by_id))
        .route("/", post(roles_controllers::create))
        .route("/{id}", put(roles_controllers::update_by_id))
        .route("/{id}", delete(roles_controllers::delete_by_id))
        .fallback(roles_controllers::not_found)
}
