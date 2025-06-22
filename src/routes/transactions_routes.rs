use crate::handlers::transactions_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn register() -> Router {
    Router::<()>::new()
        .route("/", get(transactions_handlers::get_all))
        .route("/{id}", get(transactions_handlers::get_by_id))
        .route("/", post(transactions_handlers::create))
        .route("/{id}", put(transactions_handlers::update_by_id))
        .route("/{id}", delete(transactions_handlers::delete_by_id))
        .fallback(transactions_handlers::not_found)
}
