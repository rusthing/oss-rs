// use serde::{Deserialize, Serialize};
use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display};

/// # 错误码元数据结构
///
/// 用于存储每个错误码的详细信息，包括代码字符串、名称和说明
pub struct CodeMetadata {
    /// 错误码的字符串表示
    code: &'static str,
    /// 错误码的中文名称
    name: &'static str,
    /// 错误码的说明信息
    description: &'static str,
}

// 为 RoCode trait 对象启用擦除泛型的序列化支持
// serde::Serialize带有泛型，不兼容dyn，这里使用擦除泛型的序列化
serialize_trait_object!(RoCode);

pub trait RoCode: Debug + erased_serde::Serialize {
    fn get_metadata_index(&self) -> usize;

    fn get_metadata_array(&self) -> &[CodeMetadata] {
        &CODE_METADATA
    }

    /// # 获取错误码对应的元数据
    ///
    /// ## 返回值
    /// 返回指向对应元数据的引用
    fn get_metadata(&self) -> &CodeMetadata {
        &self.get_metadata_array()[self.get_metadata_index()]
    }

    fn get_code(&self) -> &str {
        self.get_metadata().code
    }
}

impl Display for dyn RoCode {
    /// 格式化输出错误码信息
    ///
    /// # 参数
    /// * `f` - 格式化器
    ///
    /// # 返回值
    /// 格式化结果
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = self.get_metadata();
        write!(
            f,
            "RoCode {{ code: {}, name: {}, description: {} }}",
            metadata.code, metadata.name, metadata.description
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RoCodeBase {
    /// 默认成功状态码
    Success,
    /// 默认错误状态码
    Error,
    /// 参数验证失败
    ValidationError,
    /// 资源未找到
    NotFound,
    /// 权限不足
    Unauthorized,
    /// 系统内部错误
    InternalError,
}

/// 错误码元数据常量数组
///
/// 按照枚举值在定义中的顺序存储每个错误码的元数据信息
const CODE_METADATA: [CodeMetadata; 6] = [
    CodeMetadata {
        code: "SUCCESS",
        name: "成功",
        description: "操作成功",
    },
    CodeMetadata {
        code: "ERROR",
        name: "错误",
        description: "操作失败",
    },
    CodeMetadata {
        code: "VALIDATION_ERROR",
        name: "验证错误",
        description: "参数验证失败",
    },
    CodeMetadata {
        code: "NOT_FOUND",
        name: "未找到",
        description: "请求的资源未找到",
    },
    CodeMetadata {
        code: "UNAUTHORIZED",
        name: "未授权",
        description: "权限不足",
    },
    CodeMetadata {
        code: "INTERNAL_ERROR",
        name: "内部错误",
        description: "系统内部错误",
    },
];

impl RoCode for RoCodeBase {
    fn get_metadata_index(&self) -> usize {
        *self as usize
    }
}

impl RoCodeBase {
    /// # 根据错误码字符串获取对应的错误码枚举对象
    ///
    /// ## 参数
    /// * `code` - 错误码的字符串表示
    ///
    /// ## 返回值
    /// 如果找到对应的错误码，返回Some(RoCodeBase)，否则返回None
    fn new(code: &str) -> Option<Self> {
        CODE_METADATA
            .iter()
            .position(|metadata| metadata.code == code)
            .map(|index| unsafe { std::mem::transmute(index as u8) })
    }
}

impl Serialize for RoCodeBase {
    /// 序列化错误码为JSON
    ///
    /// # 参数
    /// * `serializer` - 序列化器
    ///
    /// # 返回值
    /// 序列化结果
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.get_code())
    }
}

impl<'de> Deserialize<'de> for RoCodeBase {
    /// 从JSON反序列化为错误码
    ///
    /// # 参数
    /// * `deserializer` - 反序列化器
    ///
    /// # 返回值
    /// 反序列化结果，如果代码字符串无效则返回错误
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = String::deserialize(deserializer)?;
        RoCodeBase::new(&code)
            .ok_or_else(|| serde::de::Error::custom(format!("Unknown RoCodeBase code: {}", code)))
    }
}
