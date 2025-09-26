use sha2::Digest;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

/// 获取文件扩展名
pub fn get_file_ext(file_name: &str) -> String {
    if file_name.contains('.') {
        file_name
            .split('.')
            .last()
            .unwrap()
            .to_string()
            .to_lowercase()
    } else {
        String::new()
    }
}

/// 计算文件的 SHA256 哈希值
pub fn calc_hash(path: &Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    format!("{:x}", hasher.finalize())
}

/// 检查是否为跨设备错误
pub fn is_cross_device_error(err: &io::Error) -> bool {
    match err.kind() {
        // 在 Unix 系统上，跨设备错误通常表现为 InvalidInput
        #[cfg(unix)]
        io::ErrorKind::InvalidInput => {
            // 进一步检查错误码是否为 EXDEV (18)
            if let Some(raw_os_error) = err.raw_os_error() {
                raw_os_error == 18 // EXDEV 错误码
            } else {
                false
            }
        }
        // 在 Windows 系统上，跨设备错误可能表现为 Other 或其他类型
        #[cfg(windows)]
        _ => {
            // Windows 上的跨设备错误通常包含特定的错误信息
            if let Some(raw_os_error) = err.raw_os_error() {
                raw_os_error == 17 // ERROR_NOT_SAME_DEVICE 错误码
            } else {
                false
            }
        }
        _ => false,
    }
}
