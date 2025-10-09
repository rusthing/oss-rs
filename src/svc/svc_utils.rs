use crate::svc::svc_utils::SvcError::DuplicateKey;
use log::error;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use sea_orm::DbErr;
use std::collections::HashMap;
use std::io::Error;
use thiserror::Error;

/// 正则匹配重复键错误-PostgreSQL
/// 格式: duplicate key value violates unique constraint "...", detail: Some("Key (<字段名>)=(<字段值>) already exists."), ...
static REGEX_DUPLICATE_KEY_POSTGRES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"Key \((?P<column>[^)]+)\)=\((?P<value>[^)]+)\) already exists"#).unwrap()
});

/// 正则匹配重复键错误-MySQL
/// 格式: Duplicate entry '<字段值>' for key '<字段名>'
static REGEX_DUPLICATE_KEY_MYSQL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"Duplicate entry '(?P<value>[^']+)' for key '(?P<column>[^']+)'$"#).unwrap()
});

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
pub fn handle_db_err_to_svc_error(
    db_err: DbErr,
    unique_field_hashmap: &Lazy<HashMap<&'static str, &'static str>>,
) -> SvcError {
    error!("数据库错误: {}", db_err);
    let db_err_string = format!("{:?}", db_err);

    if let Some(caps) = REGEX_DUPLICATE_KEY_POSTGRES.captures(&db_err_string) {
        // 正则匹配重复键错误-PostgreSQL
        return to_duplicate_key(caps, unique_field_hashmap);
    } else if let Some(caps) = REGEX_DUPLICATE_KEY_MYSQL.captures(&db_err_string) {
        // 正则匹配重复键错误-MySQL
        return to_duplicate_key(caps, unique_field_hashmap);
    }

    SvcError::DatabaseError(db_err)
}

/// 从正则匹配中抓取有用信息转换成重复键错误
fn to_duplicate_key(
    caps: Captures,
    unique_field_hashmap: &Lazy<HashMap<&'static str, &'static str>>,
) -> SvcError {
    let column_name = caps["column"].to_string();
    let value = caps["value"].to_string();
    let name = unique_field_hashmap
        .get(column_name.as_str())
        .unwrap()
        .to_string();
    DuplicateKey(name, value)
}
