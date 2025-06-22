use crate::handlers::roles_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn register() -> Router {
    Router::<()>::new()
        .route("/", get(roles_handlers::get_all))
        .route("/{id}", get(roles_handlers::get_by_id))
        .route("/", post(roles_handlers::create))
        .route("/{id}", put(roles_handlers::update_by_id))
        .route("/{id}", delete(roles_handlers::delete_by_id))
        .fallback(roles_handlers::not_found)
}
