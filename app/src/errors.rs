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
    ArgonError(ArgonError),
    TimeError(TimeError),
    NotFoundError(String),
    EnvironmentVariableError(VarError),
    ParseIntError(ParseIntError),
    ParseStringError(ToStrError),
    ParseRoleError,
    JwtError(JwtError),
    DatabaseError(DbErr),
    ValidationError(ValidationErrors),
    ParseJsonError(JsonRejection),
    ParseQueryError(String),
    UnauthenticatedError(String),
    ForbiddenError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, response) = match self {
            AppError::ArgonError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::PasswordHashError,
                    message: err.to_string(),
                },
            ),
            AppError::TimeError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::PasswordHashError,
                    message: err.to_string(),
                },
            ),
            AppError::EnvironmentVariableError(ref err) => (
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorResponse {
                    code: ErrorCode::EnvironmentVariableError,
                    message: err.to_string(),
                },
            ),
            AppError::NotFoundError(ref err) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: ErrorCode::NotFoundError,
                    message: String::from(err),
                },
            ),
            AppError::ParseIntError(ref err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::ParsingError,
                    message: err.to_string(),
                },
            ),
            AppError::JwtError(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::ParsingError,
                    message: err.to_string(),
                },
            ),
            AppError::DatabaseError(ref err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::DatabaseError,
                    message: err.to_string(),
                },
            ),
            AppError::ValidationError(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::ValidationError,
                    message: err.to_string(),
                },
            ),
            AppError::ParseJsonError(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::ValidationError,
                    message: err.to_string(),
                },
            ),
            AppError::UnauthenticatedError(ref err) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: ErrorCode::AuthenticationError,
                    message: String::from(err),
                },
            ),
            AppError::ParseStringError(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::ParsingError,
                    message: err.to_string(),
                },
            ),
            AppError::ForbiddenError(ref err) => (
                StatusCode::FORBIDDEN,
                ErrorResponse {
                    code: ErrorCode::ForbiddenError,
                    message: String::from(err),
                },
            ),
            AppError::ParseRoleError => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::ParsingError,
                    message: String::from("Error parsing role"),
                },
            ),
            AppError::ParseQueryError(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::ParsingError,
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
        AppError::ArgonError(err)
    }
}

impl From<TimeError> for AppError {
    fn from(err: TimeError) -> Self {
        AppError::TimeError(err)
    }
}

impl From<VarError> for AppError {
    fn from(err: VarError) -> Self {
        AppError::EnvironmentVariableError(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ParseIntError(err)
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        AppError::JwtError(err)
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::ValidationError(err)
    }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        AppError::ParseJsonError(err)
    }
}

impl From<ToStrError> for AppError {
    fn from(err: ToStrError) -> Self {
        AppError::ParseStringError(err)
    }
}
