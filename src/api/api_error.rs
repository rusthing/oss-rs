use crate::ro::ro::Ro;
use crate::svc::svc_error::SvcError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use thiserror::Error;

/// 自定义API的错误
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("参数校验错误: {0}")]
    ValidationError(String),
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    #[error("服务层错误")]
    SvcError(#[from] SvcError),
}

impl ApiError {
    fn body(&self) -> Ro<()> {
        match self {
            ApiError::ValidationError(error) => Ro::illegal_argument(error.to_string()),
            ApiError::IoError(error) => Ro::fail(error.to_string()),
            ApiError::SvcError(error) => match error {
                SvcError::NotFound() => Ro::warn(error.to_string()),
                _ => Ro::fail(error.to_string()),
            },
        }
    }
}
/// 为 ApiError 实现 ResponseError trait
impl ResponseError for ApiError {
    /// 根据异常获取状态码
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SvcError(error) => match error {
                SvcError::NotFound() => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    /// 异常时响应的方法
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.body())
    }
}
