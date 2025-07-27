use crate::AppState;
use crate::controllers::budget_configs_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(budget_configs_controller::create))
        .route("/", get(budget_configs_controller::find_all))
        .route("/{id}", get(budget_configs_controller::get_by_id))
        .route("/{id}", put(budget_configs_controller::update_by_id))
        .route("/{id}", delete(budget_configs_controller::delete_by_id))
}
