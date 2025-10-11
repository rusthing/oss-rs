use crate::cst::user_id_cst::USER_ID_HEADER_NAME;
use crate::ro::ro::Ro;
use crate::utils::svc_utils::SvcError;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use log::error;
use sea_orm::DbErr;
use thiserror::Error;

/// # 自定义API错误类型
///
/// 该枚举定义了API可能遇到的各种错误类型，包括参数校验错误、IO错误和服务层错误。
/// 通过实现ResponseError trait，这些错误可以被自动转换为HTTP响应。
///
/// ## Variants
///
/// * `ValidationError(String)` - 参数校验失败时的错误，通常返回400状态码
/// * `IoError(std::io::Error)` - IO操作错误，通常返回500状态码
/// * `SvcError(SvcError)` - 服务层错误，根据具体错误类型返回相应状态码
///
/// ## Examples
///
/// ```
/// use crate::utils::api_utils::ApiError;
///
/// let error = ApiError::ValidationError("用户名不能为空".to_string());
/// assert_eq!(format!("{}", error), "参数校验错误: 用户名不能为空");
/// ```
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("参数校验错误: {0}")]
    ValidationError(String),
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    #[error("服务层错误")]
    SvcError(#[from] SvcError),
}

/// # 为 ApiError 实现错误转换方法
///
/// 该实现定义了如何将不同类型的 API 错误转换为统一的 Ro 响应对象，
/// 以便在 HTTP 接口中返回标准化的错误信息格式。
///
/// 转换规则如下：
/// - ValidationError: 转换为非法参数错误，状态码 400
/// - IoError: 转换为系统错误，状态码 500
/// - SvcError: 根据具体的业务错误类型转换为相应的 Ro 错误对象
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
                SvcError::DeleteViolateConstraint(pk_table, foreign_key, fk_table) => {
                    Ro::warn("删除失败，有其它数据依赖于本数据".to_string()).detail(Some(format!(
                        "{} <- {} <- {}>",
                        pk_table, foreign_key, fk_table
                    )))
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
/// # 为 ApiError 实现 ResponseError trait
/// 为 ApiError 实现 ResponseError trait 以支持 Actix Web 的错误处理机制
///
/// 该实现定义了 API 错误如何转换为 HTTP 响应，包括状态码和响应体格式。
/// 根据不同的错误类型，会返回相应的 HTTP 状态码和格式化的错误信息。
impl ResponseError for ApiError {
    /// 根据异常获取状态码
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SvcError(error) => match error {
                SvcError::NotFound(_) => StatusCode::NOT_FOUND,
                SvcError::DuplicateKey(_, _) | SvcError::DeleteViolateConstraint(_, _, _) => {
                    StatusCode::OK
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    /// 异常时响应的方法
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_ro())
    }
}

/// # 从HTTP请求头中获取当前用户ID
///
/// 该函数会从请求头中提取用户ID，如果请求头中没有用户ID或格式不正确，
/// 将返回相应的ApiError错误。
///
/// ## 参数
///
/// * `req` - HTTP请求对象，包含请求头信息
///
/// ## 返回值
///
/// * `Ok(u64)` - 成功解析出的用户ID
/// * `Err(ApiError)` - 解析失败时返回的错误信息
///
/// ## 错误处理
///
/// * 如果请求头中缺少必要的用户ID参数，返回`ValidationError`
/// * 如果用户ID格式不正确，无法解析为u64类型，返回`ValidationError`
pub fn get_current_user_id(req: HttpRequest) -> actix_web::Result<u64, ApiError> {
    req.headers()
        .get(USER_ID_HEADER_NAME)
        .ok_or_else(|| ApiError::ValidationError(format!("缺少必要参数<{}>", USER_ID_HEADER_NAME)))?
        .to_str()
        .unwrap()
        .to_string()
        .parse::<u64>()
        .map_err(|_| ApiError::ValidationError(format!("参数<{}>格式不正确", USER_ID_HEADER_NAME)))
}
