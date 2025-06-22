use crate::handlers;
use axum::Router;
use axum::routing::get;

pub mod budget_config_routes;
pub mod budgets_routes;
pub mod categories_routes;
pub mod roles_routes;
pub mod transactions_routes;
pub mod user_roles_routes;
pub mod users_routes;
pub mod wallets_routes;

pub fn register() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/v1/categories", categories_routes::register())
        .nest("/api/v1/wallets", wallets_routes::register())
        .nest("/api/v1/budgets", budgets_routes::register())
        .nest("/api/v1/transactions", transactions_routes::register())
        .nest("/api/v1/roles", roles_routes::register())
        .nest("/api/v1/users", users_routes::register())
        .nest("/api/v1/budget-config", budget_config_routes::register())
        .nest("/api/v1/user-roles", user_roles_routes::register())
        .fallback(handlers::global_handlers::not_found)
}
