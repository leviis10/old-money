use crate::handlers::categories_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn register() -> Router {
    Router::<()>::new()
        .route("/", get(categories_handlers::get_all))
        .route("/{id}", get(categories_handlers::get_by_id))
        .route("/", post(categories_handlers::create))
        .route("/{id}", put(categories_handlers::update_by_id))
        .route("/{id}", delete(categories_handlers::delete_by_id))
}
