use crate::AppState;
use crate::controllers::users_controller;
use axum::Router;
use axum::routing::{delete, get, put};
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .route("/self", get(users_controller::get_self))
        .route("/self", put(users_controller::update_self))
        .route("/self", delete(users_controller::delete_self))
}
