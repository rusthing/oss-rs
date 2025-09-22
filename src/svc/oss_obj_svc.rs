use crate::config::CONFIG;
use crate::dao::oss_obj_dao;
use crate::dao::oss_obj_dao::increment_ref_count_atomic;
use crate::id_worker::ID_WORKER;
use crate::model::oss_obj::Model;
use crate::ro::ro::Ro;
use crate::svc::svc_error::SvcError;
use chrono::Utc;
use sea_orm::{DatabaseConnection, TransactionTrait};
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tempfile::NamedTempFile;

/// 根据id获取对象信息
pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<Ro<Model>, SvcError> {
    Ok(Ro::success("查询成功".to_string()).extra(oss_obj_dao::get_by_id(db, id).await?))
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
    let mut retry_count: i8 = 10;
    loop {
        let one =
            oss_obj_dao::get_by_hash_and_size(db, hash.clone().unwrap().as_str(), file_size as i64)
                .await?;
        if one.is_none() {
            break;
        }
        let result = increment_ref_count_atomic(db, one.clone().unwrap()).await?;
        if result.rows_affected() != 1 {
            retry_count -= 1;
            if retry_count <= 0 {
                return Err(SvcError::DatabaseError(sea_orm::DbErr::RecordNotUpdated));
            }
            continue;
        }
        return Ok(Ro::success("上传成功".to_string())
            .extra(oss_obj_dao::get_by_id(db, one.unwrap().id as u64).await?));
    }

    let id = ID_WORKER.get().unwrap().next_id() as i64;
    let now = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    );
    let ext = if file_name.contains('.') {
        file_name
            .split('.')
            .last()
            .unwrap()
            .to_string()
            .to_lowercase()
    } else {
        String::new()
    };
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
        name: file_name.to_string(),
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

    // 开启事务
    let tx = db.begin().await?;
    oss_obj_dao::insert(&tx, model).await?;

    fs::rename(temp_file.path(), &new_file_path)?;

    // 提交事务
    tx.commit().await?;

    Ok(Ro::success("上传成功".to_string()).extra(oss_obj_dao::get_by_id(db, id as u64).await?))
}

pub async fn remove(db: &DatabaseConnection, obj_id: u64) -> Result<Ro<()>, SvcError> {
    let model = oss_obj_dao::get_by_id(db, obj_id).await?;
    if model.is_none() {
        return Err(SvcError::NotFound());
    }
    let model = model.unwrap();
    let id = model.id as u64;

    // 开启事务
    let exec_result = oss_obj_dao::decrement_ref_count_atomic(db, model).await?;
    if exec_result.rows_affected() != 1 {
        return Err(SvcError::DatabaseError(sea_orm::DbErr::RecordNotUpdated));
    }
    let one = oss_obj_dao::get_by_id(db, id).await?;
    if one.is_none() {
        return Err(SvcError::NotFound());
    }
    let one = one.unwrap();
    if one.ref_count <= 0 {
        let tx = db.begin().await?;
        oss_obj_dao::delete(db, one.clone()).await?;
        let path = one.path.unwrap();
        fs::remove_file(path)?;
        tx.commit().await?;
    }
    Ok(Ro::success("删除成功".to_string()))
}

// 修改 download 函数中的文件读取部分
pub async fn download(
    db: &DatabaseConnection,
    obj_id: u64,
    ext: String,
    start: Option<u64>,
    mut end: Option<u64>,
) -> Result<(String, u64, u64, Vec<u8>, Option<u64>, Option<u64>), SvcError> {
    let ro = get_by_id(db, obj_id).await?;

    // 获取文件路径
    let model = ro.extra.ok_or_else(|| SvcError::NotFound())?;

    if model.ext.unwrap().as_str() != &ext {
        return Err(SvcError::NotFound());
    }

    let file_path = model.path.unwrap();

    // 读取文件指定范围内容
    let mut file = File::open(&file_path)?;
    let mut content = Vec::new();

    let file_size = file.metadata()?.len();
    let mut length = file_size;
    if start.is_some() && end.is_none() {
        end = Some(length - 1);
    }
    if let (Some(start_pos), Some(end_pos)) = (start, end) {
        file.seek(SeekFrom::Start(start_pos))?;
        length = end_pos - start_pos + 1;
        let size = CONFIG.get().unwrap().oss.download_buffer_size.as_u64();
        if length > size {
            length = size;
            end = Some(start_pos + length - 1);
        }
        content.resize(length as usize, 0);
        file.read_exact(&mut content)?;
    } else {
        file.read_to_end(&mut content)?;
    }

    Ok((model.name, file_size, length, content, start, end))
}
