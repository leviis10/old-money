use crate::AppState;
use crate::controllers::budget_config_controllers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(budget_config_controllers::get_all))
        .route("/{id}", get(budget_config_controllers::get_by_id))
        .route("/", post(budget_config_controllers::create))
        .route("/{id}", put(budget_config_controllers::update_by_id))
        .route("/{id}", delete(budget_config_controllers::delete_by_id))
        .fallback(budget_config_controllers::not_found)
}
