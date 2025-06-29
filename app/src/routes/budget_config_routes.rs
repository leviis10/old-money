use crate::AppState;
use crate::handlers::budget_config_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(budget_config_handlers::get_all))
        .route("/{id}", get(budget_config_handlers::get_by_id))
        .route("/", post(budget_config_handlers::create))
        .route("/{id}", put(budget_config_handlers::update_by_id))
        .route("/{id}", delete(budget_config_handlers::delete_by_id))
        .fallback(budget_config_handlers::not_found)
}
