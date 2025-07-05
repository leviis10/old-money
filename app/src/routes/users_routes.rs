use crate::AppState;
use crate::controllers::users_controllers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(users_controllers::get_all))
        .route("/{id}", get(users_controllers::get_by_id))
        .route("/", post(users_controllers::create))
        .route("/{id}", put(users_controllers::update_by_id))
        .route("/{id}", delete(users_controllers::delete_by_id))
        .fallback(users_controllers::not_found)
}
