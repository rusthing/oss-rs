use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum RoResult {
    Success,
    IllegalArgument,
    Warn,
    Fail,
}

struct EnumMetadata {
    id: i8,
    name: &'static str,
    note: &'static str,
}

const ENUM_METADATA: [EnumMetadata; 4] = [
    EnumMetadata {
        id: 1,
        name: "成功",
        note: "运行正常",
    },
    EnumMetadata {
        id: -1,
        name: "参数错误",
        note: "传递的参数有问题",
    },
    EnumMetadata {
        id: -2,
        name: "警告",
        note: "用户方面的错误",
    },
    EnumMetadata {
        id: -3,
        name: "失败",
        note: "系统方面的异常",
    },
];

impl RoResult {
    fn metadata(&self) -> &EnumMetadata {
        &ENUM_METADATA[*self as usize]
    }

    pub fn from_id(id: i8) -> Option<Self> {
        ENUM_METADATA
            .iter()
            .position(|metadata| metadata.id == id)
            .map(|index| unsafe { std::mem::transmute(index as u8) })
    }
}

impl fmt::Display for RoResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = self.metadata();
        write!(
            f,
            "RoResult {{ index: {}, id: {}, name: {}, note: {} }}",
            *self as usize, metadata.id, metadata.name, metadata.note
        )
    }
}

impl Serialize for RoResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8(self.metadata().id)
    }
}

impl<'de> Deserialize<'de> for RoResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = i8::deserialize(deserializer)?;
        RoResult::from_id(id)
            .ok_or_else(|| serde::de::Error::custom(format!("Unknown RoResult id: {}", id)))
    }
}
