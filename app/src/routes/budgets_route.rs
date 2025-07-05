use crate::AppState;
use crate::controllers::budgets_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(budgets_controller::get_all))
        .route("/{id}", get(budgets_controller::get_by_id))
        .route("/", post(budgets_controller::create))
        .route("/{id}", put(budgets_controller::update_by_id))
        .route("/{id}", delete(budgets_controller::delete_by_id))
        .fallback(budgets_controller::not_found)
}
