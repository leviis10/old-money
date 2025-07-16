use crate::dto::response::global::error_response::{ErrorCode, ErrorResponse};
use argon2::password_hash::Error as ArgonError;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::http::header::ToStrError;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::Error as JwtError;
use sea_orm::DbErr;
use std::env::VarError;
use std::num::ParseIntError;
use time::error::ComponentRange as TimeError;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    Argon(ArgonError),
    Time(TimeError),
    NotFound(String),
    EnvironmentVariable(VarError),
    ParseInt(ParseIntError),
    ParseString(ToStrError),
    ParseRole,
    Jwt(JwtError),
    Database(DbErr),
    Validation(ValidationErrors),
    ParseJson(JsonRejection),
    ParseQuery(String),
    Unauthenticated(String),
    Forbidden(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, response) = match self {
            AppError::Argon(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::Hash,
                    message: err.to_string(),
                },
            ),
            AppError::Time(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::Hash,
                    message: err.to_string(),
                },
            ),
            AppError::EnvironmentVariable(ref err) => (
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorResponse {
                    code: ErrorCode::EnvironmentVariable,
                    message: err.to_string(),
                },
            ),
            AppError::NotFound(ref err) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: ErrorCode::NotFound,
                    message: String::from(err),
                },
            ),
            AppError::ParseInt(ref err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::Parse,
                    message: err.to_string(),
                },
            ),
            AppError::Jwt(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::Parse,
                    message: err.to_string(),
                },
            ),
            AppError::Database(ref err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::Database,
                    message: err.to_string(),
                },
            ),
            AppError::Validation(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::Validation,
                    message: err.to_string(),
                },
            ),
            AppError::ParseJson(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::Validation,
                    message: err.to_string(),
                },
            ),
            AppError::Unauthenticated(ref err) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: ErrorCode::Authentication,
                    message: String::from(err),
                },
            ),
            AppError::ParseString(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::Parse,
                    message: err.to_string(),
                },
            ),
            AppError::Forbidden(ref err) => (
                StatusCode::FORBIDDEN,
                ErrorResponse {
                    code: ErrorCode::Authorization,
                    message: String::from(err),
                },
            ),
            AppError::ParseRole => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::Parse,
                    message: String::from("Error parsing role"),
                },
            ),
            AppError::ParseQuery(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::Parse,
                    message: String::from(err),
                },
            ),
        };

        tracing::error!("Error: {:?}", self);
        (status, response).into_response()
    }
}

impl From<ArgonError> for AppError {
    fn from(err: ArgonError) -> Self {
        AppError::Argon(err)
    }
}

impl From<TimeError> for AppError {
    fn from(err: TimeError) -> Self {
        AppError::Time(err)
    }
}

impl From<VarError> for AppError {
    fn from(err: VarError) -> Self {
        AppError::EnvironmentVariable(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ParseInt(err)
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        AppError::Jwt(err)
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::Database(err)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        AppError::ParseJson(err)
    }
}

impl From<ToStrError> for AppError {
    fn from(err: ToStrError) -> Self {
        AppError::ParseString(err)
    }
}
