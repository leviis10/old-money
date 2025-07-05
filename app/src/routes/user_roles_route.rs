use crate::AppState;
use crate::controllers::user_roles_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(user_roles_controller::get_all))
        .route("/{id}", get(user_roles_controller::get_by_id))
        .route("/", post(user_roles_controller::create))
        .route("/{id}", put(user_roles_controller::update_by_id))
        .route("/{id}", delete(user_roles_controller::delete_by_id))
        .fallback(user_roles_controller::not_found)
}
