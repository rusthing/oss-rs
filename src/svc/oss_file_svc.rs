use crate::dao::oss_bucket_dao::OssBucketDao;
use crate::dao::oss_obj_dao::OssObjDao;
use crate::dao::oss_obj_ref_dao::OssObjRefDao;
use crate::db::DB_CONN;
use crate::env::ENV;
use crate::id_worker::ID_WORKER;
use crate::model::{oss_obj, oss_obj_ref};
use crate::ro::ro::Ro;
use crate::settings::SETTINGS;
use crate::utils::file_utils::{get_file_ext, is_cross_device_error};
use crate::base::svc::svc_error::SvcError;
use crate::utils::time_utils::get_current_timestamp;
use crate::vo::oss_obj_ref::OssObjRefVo;
use chrono::{Local, TimeZone};
use sea_orm::{IntoActiveModel, TransactionTrait};
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tempfile::NamedTempFile;

/// # 上传文件到指定的存储桶中
///
/// 该函数负责将临时文件上传到系统中，并根据文件的哈希值和大小判断是否已存在相同文件。
/// 如果文件已存在，则直接创建引用关系；否则将文件移动到指定位置并创建新的对象记录。
///
/// ## 参数
/// * `bucket` - 存储桶名称
/// * `file_name` - 原始文件名
/// * `file_size` - 文件大小（字节）
/// * `hash` - 文件哈希值
/// * `temp_file` - 包含文件内容的临时文件
///
/// ## 返回值
/// * `Ok(Ro<OssObjRefVo>)` - 上传成功，返回包含文件引用信息的结果对象
/// * `Err(SvcError)` - 上传失败，返回错误信息
///
/// ## 错误处理
/// * 如果存储桶不存在，返回警告信息
/// * 如果文件操作或数据库操作失败，返回相应错误
pub async fn upload(
    bucket: &str,
    file_name: &str,
    file_size: usize,
    hash: &str,
    temp_file: NamedTempFile,
) -> Result<Ro<OssObjRefVo>, SvcError> {
    let db = DB_CONN.get().unwrap();

    let one_bucket = match OssBucketDao::get_by_name(bucket, db).await? {
        Some(bucket) => bucket,
        None => return Ok(Ro::warn(format!("未找到存储桶<{}>", bucket))),
    };

    // 开启事务
    let tx = db.begin().await?;
    let now = get_current_timestamp();
    let one = OssObjDao::get_by_hash_and_size(hash, file_size as i64, &tx).await?;
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
        OssObjDao::insert(
            oss_obj::Model {
                id,
                size: file_size as i64,
                hash: hash.to_string(),
                path: new_file_path.to_string(),
                url,
                is_completed,
                ..Default::default()
            }
            .into_active_model(),
            &tx,
        )
        .await?;
        (id, new_file_path)
    };

    // 新增对象引用
    let obj_ref_model = OssObjRefDao::insert(
        oss_obj_ref::Model {
            name: file_name.to_string(),
            bucket_id: one_bucket.id,
            obj_id,
            ext,
            ..Default::default()
        }
        .into_active_model(),
        &tx,
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

    let one = OssObjRefDao::get_by_id(obj_ref_model.id, db).await?;
    Ok(Ro::success("上传成功".to_string()).extra(one.map(OssObjRefVo::from)))
}

/// # 下载文件
///
/// 该函数负责根据对象引用ID下载文件内容，支持断点续传功能，可以指定下载文件的特定范围
///
/// ## 参数
/// * `obj_ref_id` - 对象引用ID
/// * `ext` - 文件扩展名
/// * `start` - 下载起始位置（可选）
/// * `end` - 下载结束位置（可选）
///
/// ## 返回值
/// * `Ok((String, u64, u64, Vec<u8>, Option<u64>, Option<u64>))` - 下载成功，返回元组包含：
///   - 文件名
///   - 文件总大小
///   - 实际读取长度
///   - 文件内容数据
///   - 起始位置
///   - 结束位置
/// * `Err(SvcError)` - 下载失败，返回错误信息
///
/// ## 错误处理
/// * 如果对象引用不存在或扩展名不匹配，返回 NotFound 错误
/// * 如果文件读取过程中发生错误，返回相应错误
pub async fn download(
    obj_ref_id: u64,
    ext: String,
    start: Option<u64>,
    mut end: Option<u64>,
) -> Result<(String, u64, u64, Vec<u8>, Option<u64>, Option<u64>), SvcError> {
    let db = DB_CONN.get().unwrap();
    let one = OssObjRefDao::get_by_id(obj_ref_id as i64, db).await?;
    let (obj_ref_model, _, obj_model) =
        one.ok_or(SvcError::NotFound(format!("id: {}", obj_ref_id)))?;
    // 扩展名不对也不行
    if obj_ref_model.ext.as_str() != &ext {
        return Err(SvcError::NotFound(format!("id: {}", obj_ref_id)));
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

/// # 删除文件
///
/// 该函数负责根据对象引用ID删除文件。删除过程采用事务机制，确保数据一致性。
/// 删除流程包括：先删除对象引用记录，然后检查该对象是否还有其他引用，
/// 如果没有其他引用则同时删除对象记录和实际存储的文件。
///
/// ## 参数
/// * `obj_ref_id` - 对象引用ID
///
/// ## 返回值
/// * `Ok(Ro<()>)` - 删除成功，返回成功结果对象
/// * `Err(SvcError)` - 删除失败，返回错误信息
///
/// ## 错误处理
/// * 如果对象引用不存在，返回 NotFound 错误
/// * 如果文件删除或数据库操作失败，返回相应错误
///
/// ## 事务说明
/// 整个删除过程在一个数据库事务中执行，确保引用计数和文件删除的一致性
pub async fn del(obj_ref_id: u64) -> Result<Ro<()>, SvcError> {
    let db = DB_CONN.get().unwrap();
    // 开启事务
    let tx = db.begin().await?;

    let one = OssObjRefDao::get_by_id(obj_ref_id as i64, &tx).await?;
    let (oss_obj_ref_model, _, oss_obj_model) =
        one.ok_or(SvcError::NotFound(format!("id: {}", obj_ref_id)))?;

    // 删除对象引用
    OssObjRefDao::delete(oss_obj_ref_model.into_active_model(), &tx).await?;

    // 如果对象没有引用，则删除对象
    if OssObjRefDao::count_by_obj_id(oss_obj_model.id, &tx).await? == 0 {
        let path = oss_obj_model.path.clone();

        OssObjDao::delete(oss_obj_model.into_active_model(), &tx).await?;

        // 删除文件
        fs::remove_file(path)?;
    }

    // 提交事务
    tx.commit().await?;

    Ok(Ro::success("删除成功".to_string()))
}
