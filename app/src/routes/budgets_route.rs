use crate::AppState;
use crate::controllers::budgets_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(budgets_controller::create))
        .route("/", get(budgets_controller::find_all))
        .route("/{id}", get(budgets_controller::get_by_id))
        .route("/{id}", put(budgets_controller::update_by_id))
        .route("/{id}", delete(budgets_controller::delete_by_id))
}
