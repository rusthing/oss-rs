use crate::ro::ro_result::RoResult;
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ro<T> {
    pub result: RoResult,
    pub msg: String,
    pub timestamp: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<T>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl<T> Ro<T> {
    pub fn new(result: RoResult, msg: String) -> Self {
        Ro {
            result,
            msg,
            timestamp: Utc::now().timestamp_millis() as u64,
            extra: None,
            detail: None,
            code: None,
        }
    }
    pub fn success(msg: String) -> Self {
        Self::new(RoResult::Success, msg)
    }
    pub fn illegal_argument(msg: String) -> Self {
        Self::new(RoResult::IllegalArgument, msg)
    }
    pub fn warn(msg: String) -> Self {
        Self::new(RoResult::Warn, msg)
    }
    pub fn fail(msg: String) -> Self {
        Self::new(RoResult::Fail, msg)
    }
    pub fn msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }
    pub fn extra(mut self, extra: Option<T>) -> Self {
        self.extra = extra;
        self
    }
    pub fn detail(mut self, detail: Option<String>) -> Self {
        self.detail = detail;
        self
    }
    pub fn code(mut self, code: Option<String>) -> Self {
        self.code = code;
        self
    }
    pub fn get_extra(self) -> Option<T> {
        self.extra
    }
}
