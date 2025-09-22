use log::error;
use sea_orm::DbErr;
use std::io::Error;
use thiserror::Error;

/// 自定义SVC的错误
#[derive(Debug, Error)]
pub enum SvcError {
    #[error("找不到数据")]
    NotFound(),
    #[error("IO错误: {0}")]
    IoError(#[from] Error),
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] DbErr),
}
