use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::controllers::auth_controller::register,
    crate::controllers::auth_controller::login
))]
pub struct ApiDoc;
