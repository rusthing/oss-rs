use crate::dao::{oss_bucket_dao, oss_obj_dao, oss_obj_ref_dao};
use crate::db::DB_CONN;
use crate::env::ENV;
use crate::id_worker::ID_WORKER;
use crate::model::{oss_obj, oss_obj_ref};
use crate::ro::ro::Ro;
use crate::settings::SETTINGS;
use crate::svc::svc_error::SvcError;
use crate::utils::file_utils::{get_file_ext, is_cross_device_error};
use crate::utils::time_utils::get_current_timestamp;
use crate::vo::oss_obj_ref::OssObjRefVo;
use chrono::{Local, TimeZone};
use sea_orm::TransactionTrait;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tempfile::NamedTempFile;

/// 根据id获取对象信息
pub async fn get_by_id(obj_ref_id: u64) -> Result<Ro<OssObjRefVo>, SvcError> {
    let db = DB_CONN.get().unwrap();
    let one = oss_obj_ref_dao::get_by_id(db, obj_ref_id as i64).await?;
    Ok(Ro::success("查询成功".to_string()).extra(match one {
        Some(one) => Some(OssObjRefVo::from(one)),
        _ => return Err(SvcError::NotFound()),
    }))
}

/// 上传对象
pub async fn upload(
    bucket: &str,
    file_name: &str,
    file_size: usize,
    hash: &str,
    temp_file: NamedTempFile,
) -> Result<Ro<OssObjRefVo>, SvcError> {
    let db = DB_CONN.get().unwrap();

    let one_bucket = match oss_bucket_dao::get_by_name(db, bucket).await? {
        Some(bucket) => bucket,
        None => return Ok(Ro::warn("未找到存储桶".to_string())),
    };

    // 开启事务
    let tx = db.begin().await?;
    let now = get_current_timestamp();
    let one = oss_obj_dao::get_by_hash_and_size(&tx, hash, file_size as i64).await?;
    let ext = get_file_ext(file_name);
    // 判断对象是否存在
    let obj_existed = one.is_some();
    let (obj_id, new_file_path) = if obj_existed {
        // 如果已经上传过该文件，则直接返回之前的对象ID和存放路径
        let one = one.unwrap();
        (one.id, one.path)
    } else {
        // 如果未上传过该文件，则新增对象，并返回新对象ID和新文件的存放路径
        // 生成对象ID
        let id = ID_WORKER.get().unwrap().next_id() as i64;
        let name = format!("{}.{}", id, ext);
        let url = format!("/oss/obj/preview/{}", name);
        let is_completed = true;
        // 根据当前时间，创建yyyy/MM/dd/HH的目录，并将文件存入此目录中
        let datetime = Local.timestamp_opt((now / 1000) as i64, 0).unwrap();
        let settings_oss = SETTINGS.get().unwrap().oss.clone();

        let date_path = datetime.format(&settings_oss.file_dir_format).to_string();

        let storage_dir = ENV
            .get()
            .unwrap()
            .app_dir
            .join(&settings_oss.file_root_dir)
            .join(bucket.to_string())
            .join(&date_path);
        fs::create_dir_all(&storage_dir)?;
        let new_file_path = storage_dir
            .join(&name)
            .as_path()
            .to_str()
            .unwrap()
            .to_string();

        // 新增对象
        oss_obj_dao::insert(
            &tx,
            oss_obj::Model {
                id,
                name: file_name.to_string(),
                size: file_size as i64,
                hash: hash.to_string(),
                path: new_file_path.to_string(),
                url,
                is_completed,
                ..Default::default()
            },
        )
        .await?;
        (id, new_file_path)
    };

    // 新增对象引用
    let obj_ref_model = oss_obj_ref_dao::insert(
        &tx,
        oss_obj_ref::Model {
            name: file_name.to_string(),
            bucket_id: one_bucket.id,
            obj_id,
            ext,
            ..Default::default()
        },
    )
    .await?;

    if obj_existed {
        // 如果对象已经存在，则直接关闭并删除临时文件
        temp_file.close()?;
    } else {
        // 如果对象不存在，则移动临时文件到目标目录中
        let (source, destination) = (temp_file.path(), &new_file_path);
        match fs::rename(source, destination) {
            Ok(_) => {}
            Err(e) if is_cross_device_error(&e) => {
                // 如果为跨设备错误，使用 copy + remove 替代
                fs::copy(source, destination)?;
                temp_file.close()?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    // 提交事务
    tx.commit().await?;

    let one = oss_obj_ref_dao::get_by_id(db, obj_ref_model.id).await?;
    Ok(Ro::success("上传成功".to_string()).extra(one.map(OssObjRefVo::from)))
}

// 下载
pub async fn download(
    obj_ref_id: u64,
    ext: String,
    start: Option<u64>,
    mut end: Option<u64>,
) -> Result<(String, u64, u64, Vec<u8>, Option<u64>, Option<u64>), SvcError> {
    let db = DB_CONN.get().unwrap();
    let one = oss_obj_ref_dao::get_by_id(db, obj_ref_id as i64).await?;
    let (obj_ref_model, _, obj_model) = one.ok_or(SvcError::NotFound())?;
    // 扩展名不对也不行
    if obj_ref_model.ext.as_str() != &ext {
        return Err(SvcError::NotFound());
    }

    // 读取文件指定范围内容
    let mut file = File::open(&obj_model.path)?;
    let mut content = Vec::new();

    let file_size = file.metadata()?.len();
    let mut length = file_size;
    if start.is_some() && end.is_none() {
        end = Some(length - 1);
    }
    if let (Some(start_pos), Some(end_pos)) = (start, end) {
        file.seek(SeekFrom::Start(start_pos))?;
        length = end_pos - start_pos + 1;
        let size = SETTINGS.get().unwrap().oss.download_buffer_size.as_u64();
        if length > size {
            length = size;
            end = Some(start_pos + length - 1);
        }
        content.resize(length as usize, 0);
        file.read_exact(&mut content)?;
    } else {
        file.read_to_end(&mut content)?;
    }

    Ok((obj_ref_model.name, file_size, length, content, start, end))
}

pub async fn remove(obj_ref_id: u64) -> Result<Ro<()>, SvcError> {
    let db = DB_CONN.get().unwrap();
    // 开启事务
    let tx = db.begin().await?;

    let one = oss_obj_ref_dao::get_by_id(&tx, obj_ref_id as i64).await?;
    let (obj_ref_model, _, obj_model) = one.ok_or(SvcError::NotFound())?;

    // 删除对象引用
    oss_obj_ref_dao::delete(&tx, obj_ref_model).await?;

    // 如果对象没有引用，则删除对象
    if oss_obj_ref_dao::count_by_obj_id(&tx, obj_model.id).await? == 0 {
        oss_obj_dao::delete(&tx, obj_model.clone()).await?;

        // 删除文件
        fs::remove_file(obj_model.path)?;
    }

    // 提交事务
    tx.commit().await?;

    Ok(Ro::success("删除成功".to_string()))
}
