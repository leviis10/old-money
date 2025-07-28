use crate::AppState;
use crate::controllers::categories_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(categories_controller::create))
        .route("/", get(categories_controller::find_all))
        .route("/{id}", put(categories_controller::update_by_id))
        .route("/{id}", delete(categories_controller::delete_by_id))
}
