use crate::AppState;
use crate::controllers::transactions_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(transactions_controller::get_all))
        .route("/{id}", get(transactions_controller::get_by_id))
        .route("/", post(transactions_controller::create))
        .route("/{id}", put(transactions_controller::update_by_id))
        .route("/{id}", delete(transactions_controller::delete_by_id))
        .fallback(transactions_controller::not_found)
}
