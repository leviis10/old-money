use crate::AppState;
use crate::controllers::categories_controllers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(categories_controllers::get_all))
        .route("/{id}", get(categories_controllers::get_by_id))
        .route("/", post(categories_controllers::create))
        .route("/{id}", put(categories_controllers::update_by_id))
        .route("/{id}", delete(categories_controllers::delete_by_id))
}
