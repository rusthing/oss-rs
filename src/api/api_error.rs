use crate::ro::ro::Ro;
use crate::ro::ro_result::RoResult;
use crate::svc::svc_error::SvcError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use std::error::Error;
use thiserror::Error;

/// 自定义API的错误
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("参数校验错误: {0}")]
    ValidationError(String),
    #[error("服务层错误")]
    SvcError(#[from] SvcError),
}

/// 为 ApiError 实现 ResponseError trait
impl ResponseError for ApiError {
    /// 根据异常获取状态码
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::SvcError(error) => match error {
                SvcError::NotFound() => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    /// 异常时响应的方法
    fn error_response(&self) -> HttpResponse {
        let ro_result = match self {
            ApiError::ValidationError(_) => RoResult::IllegalArgument,
            ApiError::SvcError(error) => match error {
                SvcError::NotFound() => RoResult::Warn,
                _ => RoResult::Fail,
            },
        };
        let mut body: Ro<()> = Ro::new(ro_result, self.to_string());
        if let ApiError::SvcError(_) = self {
            body = body.detail(Some(self.source().unwrap().to_string()));
        }
        HttpResponse::build(self.status_code()).json(&body)
    }
}
