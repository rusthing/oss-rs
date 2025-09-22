use crate::dao::oss_obj_dao;
use crate::id_worker::ID_WORKER;
use crate::model::oss_obj::Model;
use crate::ro::ro::Ro;
use crate::svc::svc_error::SvcError;
use chrono::Utc;
use sea_orm::{DatabaseConnection, TransactionTrait};
use std::fs;
use std::fs::File;
use std::io::Read;
use tempfile::NamedTempFile;

/// 根据id获取对象信息
pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<Ro<Model>, SvcError> {
    let result = oss_obj_dao::get_by_id(db, id).await?;
    Ok(Ro::success("查询成功".to_string()).extra(result))
}

/// 上传对象
pub async fn upload(
    db: &DatabaseConnection,
    bucket: &str,
    file_name: &str,
    file_size: usize,
    hash: Option<String>,
    temp_file: NamedTempFile,
) -> Result<Ro<Model>, SvcError> {
    // 开启事务
    let txn = db.begin().await?;
    // let result = oss_obj_dao::get_by_id(db, id).await?;
    let id = ID_WORKER.get().unwrap().next_id() as i64;
    let now = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    );
    let ext = file_name
        .split(".")
        .last()
        .unwrap()
        .to_string()
        .to_lowercase();
    let name = format!("{}.{}", id, ext);
    let url = Some(format!("/oss/obj/preview/{}", name));
    let ref_count = 1;
    let is_completed = true;
    // 根据当前时间，创建yyyy/MM/dd/HH的目录，并将文件存入此目录中
    let datetime = Utc::now();
    let date_path = datetime.format("%Y/%m/%d/%H").to_string();
    let storage_dir = std::path::Path::new("storage").join(&date_path);
    fs::create_dir_all(&storage_dir)?;
    let new_file_path = storage_dir.join(&name);

    let model = Model {
        id,
        bucket: bucket.to_string(),
        name,
        ext: Some(ext),
        size: Some(file_size as i64),
        hash,
        path: Some(new_file_path.as_path().to_str().unwrap().to_string()),
        url,
        ref_count,
        is_completed,
        create_timestamp: now,
        update_timestamp: now,
        creator_id: None,
        updator_id: None,
    };
    oss_obj_dao::insert(&txn, model).await?;

    fs::rename(temp_file.path(), &new_file_path)?;

    // 提交事务
    txn.commit().await?;
    Ok(Ro::success("上传成功".to_string()))
}

pub async fn download(
    db: &DatabaseConnection,
    obj_id: u64,
    ext: String,
) -> Result<(String, Vec<u8>), SvcError> {
    let ro = get_by_id(db, obj_id).await?;

    // 获取文件路径
    let model = ro.extra.ok_or_else(|| SvcError::NotFound())?;

    if model.ext.as_ref().unwrap() != &ext {
        return Err(SvcError::NotFound());
    }

    let file_path = model.path.unwrap();

    // 读取文件内容
    let mut file = File::open(&file_path)?;

    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok((model.name, content))
}
