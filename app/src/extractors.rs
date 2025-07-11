use crate::errors::AppError;
use axum::Json;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let payload = Json::<T>::from_request(req, state).await?;
        payload.validate()?;

        Ok(ValidatedJson(payload.0))
    }
}
