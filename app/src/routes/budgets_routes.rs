use crate::AppState;
use crate::handlers::budgets_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(budgets_handlers::get_all))
        .route("/{id}", get(budgets_handlers::get_by_id))
        .route("/", post(budgets_handlers::create))
        .route("/{id}", put(budgets_handlers::update_by_id))
        .route("/{id}", delete(budgets_handlers::delete_by_id))
        .fallback(budgets_handlers::not_found)
}
