// `api_client` 保持公开，与已发布到 registry 的 1.7.2 接口一致（下游按
// `oss_api_client::api_client::get_oss_api_client` 引用）；同时用 `pub use`
// 把内容提到 crate 根，兼容按 `oss_api_client::get_oss_api_client` 引用的写法。
pub mod api_client;

pub use api_client::*;

pub use oss_api::*;
