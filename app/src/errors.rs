use crate::dto::response::global::error_response::{ErrorCode, ErrorResponse};
use argon2::password_hash::Error as ArgonError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::env::VarError;
use std::num::ParseIntError;
use time::error::ComponentRange as TimeError;
use jsonwebtoken::errors::Error as JwtError;
use sea_orm::DbErr;

#[derive(Debug)]
pub enum AppError {
    ArgonError(ArgonError),
    TimeError(TimeError),
    NotFound(String),
    VarError(VarError),
    ParseIntError(ParseIntError),
    JwtError(JwtError),
    DbErr(DbErr)
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
            AppError::VarError(ref err) => (
                StatusCode::SERVICE_UNAVAILABLE,
                ErrorResponse {
                    code: ErrorCode::MissingEnvironmentVariable,
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
            AppError::ParseIntError(ref err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::ParsingError,
                    message: err.to_string()
                }
            ),
            AppError::JwtError(ref err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::ParsingError,
                    message: err.to_string()
                }
            ),
            AppError::DbErr(ref err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::DatabaseError,
                    message: err.to_string()
                }
            ) 
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
        AppError::VarError(err)
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
        AppError::DbErr(err)
    }
}
