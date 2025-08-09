use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::auth_controller::register,
        crate::controllers::auth_controller::login,
        crate::controllers::auth_controller::refresh,

        crate::controllers::budget_configs_controller::find_all,
        crate::controllers::budget_configs_controller::get_by_id,
        crate::controllers::budget_configs_controller::update_by_id,
        crate::controllers::budget_configs_controller::delete_by_id,

        crate::controllers::categories_controller::create,
        crate::controllers::categories_controller::find_all,
        crate::controllers::categories_controller::update_by_id,
        crate::controllers::categories_controller::delete_by_id,

        crate::controllers::users_controller::get_self,
        crate::controllers::users_controller::update_self,
        crate::controllers::users_controller::delete_self,

        crate::controllers::wallets_controller::create,
        crate::controllers::wallets_controller::find_all,
        crate::controllers::wallets_controller::get_by_id,
        crate::controllers::wallets_controller::update_by_id,
        crate::controllers::wallets_controller::delete_by_id,

        crate::controllers::budgets_controller::create,
        crate::controllers::budgets_controller::find_all,
        crate::controllers::budgets_controller::get_by_id,
        crate::controllers::budgets_controller::update_by_id,
        crate::controllers::budgets_controller::delete_by_id,

        crate::controllers::transactions_controller::create,
        crate::controllers::transactions_controller::find_all,
        crate::controllers::transactions_controller::get_by_id,
        crate::controllers::transactions_controller::update_by_id,
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }
}
