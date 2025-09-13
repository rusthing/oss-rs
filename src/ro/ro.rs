use crate::ro::ro_result::RoResult;

pub struct Ro {
    pub result: RoResult,
    pub msg: String,
    pub extra: String,
    pub detail: String,
    pub code: String,
}

impl Ro {
    pub fn new() -> Self {
        Ro {
            result: RoResult::Success,
            msg: "".to_string(),
            extra: "".to_string(),
            detail: "".to_string(),
            code: "".to_string(),
        }
    }
}
