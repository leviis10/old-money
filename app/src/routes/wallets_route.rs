use crate::AppState;
use crate::controllers::wallets_controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(wallets_controller::create))
        .route("/", get(wallets_controller::find_all))
        .route("/{id}", get(wallets_controller::get_by_id))
        .route("/{id}", put(wallets_controller::update_by_id))
        .route("/{id}", delete(wallets_controller::delete_by_id))
}
