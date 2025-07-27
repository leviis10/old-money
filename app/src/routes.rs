use crate::{AppState, controllers};
use axum::Router;
use std::sync::Arc;

pub mod auth_route;
pub mod budget_configs_route;
pub mod budgets_route;
pub mod categories_route;
pub mod roles_route;
pub mod transactions_route;
pub mod user_roles_route;
pub mod users_route;
pub mod wallets_route;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api/v1/auth", auth_route::register())
        .nest("/api/v1/categories", categories_route::register())
        .nest("/api/v1/wallets", wallets_route::register())
        .nest("/api/v1/budgets", budgets_route::register())
        .nest("/api/v1/transactions", transactions_route::register())
        .nest("/api/v1/roles", roles_route::register())
        .nest("/api/v1/users", users_route::register())
        .nest("/api/v1/budget-configs", budget_configs_route::register())
        .nest("/api/v1/user-roles", user_roles_route::register())
        .fallback(controllers::global_controller::not_found)
}
