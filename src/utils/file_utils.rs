use sha2::Digest;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
