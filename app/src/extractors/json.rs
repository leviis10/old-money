use crate::AppState;
use crate::errors::AppError;
use axum::Json;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T> FromRequest<Arc<AppState>> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state).await?;
        payload.validate()?;

        Ok(ValidatedJson(payload))
    }
}
