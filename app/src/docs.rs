use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::auth_controller::register,
        crate::controllers::auth_controller::login,
        crate::controllers::auth_controller::refresh,
        crate::controllers::categories_controller::create,
        crate::controllers::categories_controller::get_all,
        crate::controllers::categories_controller::update_by_id,
        crate::controllers::categories_controller::delete_by_id,
        crate::controllers::users_controller::get_self,
        crate::controllers::users_controller::update_self,
        crate::controllers::users_controller::delete_self,
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
