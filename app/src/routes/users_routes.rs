use crate::AppState;
use crate::handlers::users_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(users_handlers::get_all))
        .route("/{id}", get(users_handlers::get_by_id))
        .route("/", post(users_handlers::create))
        .route("/{id}", put(users_handlers::update_by_id))
        .route("/{id}", delete(users_handlers::delete_by_id))
        .fallback(users_handlers::not_found)
}
