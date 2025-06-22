use crate::handlers::user_roles_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn register() -> Router {
    Router::<()>::new()
        .route("/", get(user_roles_handlers::get_all))
        .route("/{id}", get(user_roles_handlers::get_by_id))
        .route("/", post(user_roles_handlers::create))
        .route("/{id}", put(user_roles_handlers::update_by_id))
        .route("/{id}", delete(user_roles_handlers::delete_by_id))
        .fallback(user_roles_handlers::not_found)
}
