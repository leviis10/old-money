use crate::AppState;
use crate::controllers::budgets_controllers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(budgets_controllers::get_all))
        .route("/{id}", get(budgets_controllers::get_by_id))
        .route("/", post(budgets_controllers::create))
        .route("/{id}", put(budgets_controllers::update_by_id))
        .route("/{id}", delete(budgets_controllers::delete_by_id))
        .fallback(budgets_controllers::not_found)
}
