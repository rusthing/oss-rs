use crate::dao::oss_obj_dao;
use crate::id_worker::ID_WORKER;
use crate::model::oss_obj::Model;
use crate::ro::ro::Ro;
use sea_orm::{DatabaseConnection, DbErr};
use tempfile::NamedTempFile;

/// 根据id获取对象信息
pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<Ro<Model>, DbErr> {
    let result = oss_obj_dao::get_by_id(db, id).await?;
    Ok(Ro::success("查询成功".to_string()).extra(result))
}

/// 上传对象
pub async fn upload(
    db: &DatabaseConnection,
    bucket: &str,
    file_name: &str,
    file_size: usize,
    temp_file: NamedTempFile,
) -> Result<Ro<Model>, DbErr> {
    // let result = oss_obj_dao::get_by_id(db, id).await?;
    let id = ID_WORKER.get().unwrap().next_id() as i64;
    let now = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    );
    let name = file_name.to_string();
    let ext = Some(
        file_name
            .split(".")
            .last()
            .unwrap()
            .to_string()
            .to_lowercase(),
    );
    let hash = None;
    let url = Some("/oss/obj/preview/".to_owned() + file_name);
    let ref_count = 1;
    let is_completed = true;
    let model = Model {
        id,
        bucket: bucket.to_string(),
        name,
        ext,
        size: Some(file_size as i64),
        hash,
        path: Some(temp_file.path().to_str().unwrap().to_string()),
        url,
        ref_count,
        is_completed,
        create_timestamp: now,
        update_timestamp: now,
        creator_id: None,
        updator_id: None,
    };
    oss_obj_dao::insert(db, model).await?;
    Ok(Ro::success("上传成功".to_string()))
}
