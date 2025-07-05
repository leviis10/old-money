use crate::AppState;
use crate::controllers::transactions_controllers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(transactions_controllers::get_all))
        .route("/{id}", get(transactions_controllers::get_by_id))
        .route("/", post(transactions_controllers::create))
        .route("/{id}", put(transactions_controllers::update_by_id))
        .route("/{id}", delete(transactions_controllers::delete_by_id))
        .fallback(transactions_controllers::not_found)
}
