use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};
use diesel::r2d2::{Error as R2D2Error, PoolError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
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

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    AppError::UnprocessableEntity(vec![message])
                } else {
                    log::error!("Unexpected diesel error {:?}", info.message());
                    AppError::InternalServerError
                }
            }
            DieselError::NotFound => AppError::NotFound,
            _ => AppError::InternalServerError,
        }
    }
}

impl From<BlockingError> for AppError {
    fn from(err: BlockingError) -> Self {
        log::error!(
            "Error during running a blocking call in background: {:?}",
            err.to_string()
        );
        AppError::InternalServerError
    }
}
