use crate::AppState;
use crate::controllers::categories_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(categories_controller::get_all))
        .route("/", post(categories_controller::create))
        .route("/{1}", get(categories_controller::get_by_name))
        .route("/{1}", put(categories_controller::update_by_id))
        .route("/{1}", delete(categories_controller::delete_by_id))
}
