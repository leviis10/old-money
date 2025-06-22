use crate::handlers::wallets_handlers;
use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn register() -> Router {
    Router::<()>::new()
        .route("/", get(wallets_handlers::get_all))
        .route("/{id}", get(wallets_handlers::get_by_id))
        .route("/", post(wallets_handlers::create))
        .route("/{id}", put(wallets_handlers::update_by_id))
        .route("/{id}", delete(wallets_handlers::delete_by_id))
        .fallback(wallets_handlers::not_found)
}
