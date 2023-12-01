use actix_web::{HttpRequest, HttpResponse};

use crate::errors::AppError;

pub async fn test(req: HttpRequest) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(()))
}
