use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::r2d2::{Error as R2D2Error, PoolError};
use thiserror::Error as ThisError;

#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum AppError {
    /// 401
    #[error("Unauthorized: {}", _0)]
    Unauthorized(String),

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 422
    #[error("Unprocessable Entity: {:?}", _0)]
    UnprocessableEntity(Vec<String>),

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Request Error")]
    RequestError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::Forbidden => HttpResponse::Forbidden().finish(),
            AppError::NotFound => HttpResponse::NotFound().finish(),
            AppError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
            AppError::InternalServerError => HttpResponse::InternalServerError().finish(),
            AppError::DeserializeError => unreachable!(),
            AppError::RequestError => unreachable!(),
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DeserializeError => unreachable!(),
            AppError::RequestError => unreachable!(),
        }
    }
}

impl From<R2D2Error> for AppError {
    fn from(_err: R2D2Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<PoolError> for AppError {
    fn from(_err: PoolError) -> Self {
        AppError::InternalServerError
    }
}
