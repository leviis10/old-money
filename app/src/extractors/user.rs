use crate::AppState;
use crate::dto::request::users_dto::find_user_by_pk_request::FindUserByPkRequest;
use crate::entities::{roles, users};
use crate::enums::roles::Roles;
use crate::errors::AppError;
use crate::services::users_service;
use crate::utils::jwt_utils::AccessTokenClaims;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use std::sync::Arc;

pub struct User(pub users::Model, pub Vec<roles::Model>);

impl User {
    pub fn has_any_role(
        user_roles: Vec<roles::Model>,
        allowed_roles: Vec<Roles>,
    ) -> Result<(), AppError> {
        let user_roles = Roles::from_models(user_roles)?;
        let allowed = user_roles.iter().any(|role| allowed_roles.contains(role));

        if !allowed {
            return Err(AppError::Forbidden(String::from("Forbidden")));
        }

        Ok(())
    }
}

impl FromRequestParts<Arc<AppState>> for User {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let token_header = parts.headers.get(AUTHORIZATION);
        let Some(token) = token_header else {
            return Err(AppError::Unauthenticated(String::from(
                "Please authenticate",
            )));
        };

        let access_token = token.to_str()?.replace("Bearer ", "");
        let access_token_claim = AccessTokenClaims::parse(&access_token)?;
        let (found_user, roles) = users_service::find_by_pk(
            &state.db,
            FindUserByPkRequest {
                user_id: access_token_claim.sub,
            },
        )
        .await?;
        Ok(User(found_user, roles))
    }
}
