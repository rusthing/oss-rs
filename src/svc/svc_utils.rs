use crate::svc::svc_utils::SvcError::DuplicateKey;
use log::error;
use regex::Regex;
use sea_orm::DbErr;
use std::io::Error;
use thiserror::Error;

/// 自定义服务层的错误
#[derive(Debug, Error)]
pub enum SvcError {
    #[error("找不到数据: {0}")]
    NotFound(String),
    #[error("重复键错误: {0}")]
    DuplicateKey(String, String),
    #[error("IO错误: {0}")]
    IoError(#[from] Error),
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] DbErr),
}

/// 处理数据库错误，并转换为服务层错误
pub fn handle_db_err_to_svc_error(db_err: DbErr) -> SvcError {
    error!("数据库错误: {}", db_err);
    let db_err_string = format!("{:?}", db_err);

    // PostgreSQL 格式: duplicate key value violates unique constraint "...", detail: Some("Key (<字段名>)=(<字段值>) already exists."), ...
    let postgres_re =
        Regex::new(r#"Key \((?P<column>[^)]+)\)=\((?P<value>[^)]+)\) already exists"#).unwrap();
    if let Some(caps) = postgres_re.captures(&db_err_string) {
        let column_name = caps["column"].to_string();
        let value = caps["value"].to_string();
        return DuplicateKey(column_name, value);
    }

    // MySQL 格式: Duplicate entry '<字段值>' for key '<字段名>'
    let mysql_re =
        Regex::new(r#"Duplicate entry '(?P<value>[^']+)' for key '(?P<column>[^']+)'$"#).unwrap();
    if let Some(caps) = mysql_re.captures(&db_err_string) {
        let value = caps["value"].to_string();
        let column_name = caps["column"].to_string();
        return DuplicateKey(column_name, value);
    }

    SvcError::DatabaseError(db_err)
}
