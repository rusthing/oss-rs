use crate::ro::ro_result::RoResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct Ro<T> {
    pub result: RoResult,
    pub msg: String,
    #[serde(default)]
    pub extra: Option<T>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

impl<T> Ro<T> {
    pub fn new(result: RoResult, msg: String) -> Self {
        Ro {
            result,
            msg,
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
    pub fn extra(mut self, extra: T) -> Self {
        self.extra = Some(extra);
        self
    }
    pub fn detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}
