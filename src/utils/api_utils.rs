use crate::ro::ro::Ro;
use crate::utils::svc_utils::SvcError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use sea_orm::DbErr;
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
    /// 将错误转换为Ro对象
    fn to_ro(&self) -> Ro<()> {
        match self {
            ApiError::ValidationError(error) => Ro::illegal_argument(error.to_string()),
            ApiError::IoError(error) => {
                Ro::fail("磁盘异常".to_string()).detail(Some(error.to_string()))
            }
            ApiError::SvcError(error) => match error {
                SvcError::NotFound(err) => {
                    Ro::warn("找不到数据".to_string()).detail(Some(err.to_string()))
                }
                SvcError::DuplicateKey(field_name, field_value) => {
                    Ro::warn(format!("{}<{}>已存在！", field_name, field_value))
                }
                SvcError::DatabaseError(db_err) => match db_err {
                    DbErr::RecordNotUpdated => {
                        Ro::warn("未更新数据，请检查记录是否存在".to_string())
                    }
                    _ => Ro::fail("数据库错误".to_string()).detail(Some(db_err.to_string())),
                },
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
                SvcError::NotFound(_) => StatusCode::NOT_FOUND,
                SvcError::DuplicateKey(_, _) => StatusCode::OK,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    /// 异常时响应的方法
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_ro())
    }
}
