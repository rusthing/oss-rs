use bytesize::ByteSize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct OssSettings {
    /// 文件根目录
    #[serde(default = "file_root_dir_default")]
    pub file_root_dir: String,
    /// 存储文件目录格式
    #[serde(default = "file_dir_format_default")]
    pub file_dir_format: String,
    /// 上传文件限制的大小
    #[serde(default = "upload_file_limit_size_default")]
    pub upload_file_limit_size: ByteSize,
    /// 上传缓冲区大小
    #[serde(default = "upload_buffer_size_default")]
    pub upload_buffer_size: ByteSize,
    /// 下载缓冲区大小
    #[serde(default = "download_buffer_size_default")]
    pub download_buffer_size: ByteSize,
}

impl Default for OssSettings {
    fn default() -> Self {
        OssSettings {
            file_root_dir: file_root_dir_default(),
            file_dir_format: file_dir_format_default(),
            upload_file_limit_size: upload_file_limit_size_default(),
            upload_buffer_size: upload_buffer_size_default(),
            download_buffer_size: download_buffer_size_default(),
        }
    }
}

fn file_root_dir_default() -> String {
    "storage".to_string()
}

fn file_dir_format_default() -> String {
    "%Y/%m/%d/%H".to_string()
}

fn upload_file_limit_size_default() -> ByteSize {
    ByteSize::mib(300)
}
fn upload_buffer_size_default() -> ByteSize {
    ByteSize::mib(1)
}

fn download_buffer_size_default() -> ByteSize {
    ByteSize::mib(1)
}
