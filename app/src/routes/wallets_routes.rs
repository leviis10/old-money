use crate::AppState;
use crate::controllers::wallets_controllers;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(wallets_controllers::get_all))
        .route("/{id}", get(wallets_controllers::get_by_id))
        .route("/", post(wallets_controllers::create))
        .route("/{id}", put(wallets_controllers::update_by_id))
        .route("/{id}", delete(wallets_controllers::delete_by_id))
        .fallback(wallets_controllers::not_found)
}
