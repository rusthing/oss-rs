#[derive(Copy, Clone)]
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

    fn id(&self) -> i8 {
        self.metadata().id
    }
    fn name(&self) -> &'static str {
        self.metadata().name
    }

    fn note(&self) -> &'static str {
        self.metadata().note
    }
}
