use crate::dao::oss_obj_ref_dao::OssObjRefDao;
use crate::dto::oss_obj_dto::OssObjAddDto;
use crate::dto::oss_obj_ref_dto::OssObjRefAddDto;
use crate::settings::SETTINGS;
use crate::svc::oss_bucket_svc::OssBucketSvc;
use crate::svc::oss_obj_ref_svc::OssObjRefSvc;
use crate::svc::oss_obj_svc::OssObjSvc;
use crate::vo::oss_obj_ref::OssObjRefVo;
use chrono::{Local, TimeZone};
use idworker::ID_WORKER;
use robotech::db::DB_CONN;
use robotech::env::ENV;
use robotech::ro::Ro;
use robotech::svc::svc_error::SvcError;
use sea_orm::{DatabaseConnection, TransactionTrait};
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tempfile::NamedTempFile;
use wheel_rs::file_utils::{get_file_ext, is_cross_device_error};
use wheel_rs::time_utils::get_current_timestamp;

pub struct OssFileSvc;

impl OssFileSvc {
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
        current_user_id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjRefVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());

        // 开启事务
        let tx = db.begin().await?;

        // 获取存储桶
        let one_bucket = OssBucketSvc::get_by_name(bucket, Some(db)).await?;
        let one_bucket = match one_bucket.get_extra() {
            Some(bucket) => bucket,
            None => return Ok(Ro::warn(format!("未找到存储桶<{}>", bucket))),
        };

        let now = get_current_timestamp();
        let obj_vo = OssObjSvc::get_by_hash_and_size(hash, file_size as i64, Some(db))
            .await?
            .get_extra();
        let ext = get_file_ext(file_name);
        // 判断对象是否存在
        let obj_existed = obj_vo.is_some();
        let (obj_id, new_file_path) = if obj_existed {
            // 如果已经上传过该文件，则直接返回之前的对象ID和存放路径
            let obj_vo = obj_vo.unwrap();
            (obj_vo.id.parse::<u64>().unwrap(), obj_vo.path)
        } else {
            // 如果未上传过该文件，则新增对象，并返回新对象ID和新文件的存放路径
            // 生成对象ID
            let obj_id = ID_WORKER.get().unwrap().next_id();
            let name = format!("{}.{}", obj_id, ext);
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
            OssObjSvc::add(
                OssObjAddDto {
                    id: Some(obj_id.to_string()),
                    hash: Some(hash.to_string()),
                    size: Some(file_size.to_string()),
                    path: Some(new_file_path.to_string()),
                    url: Some(url),
                    is_completed: Some(is_completed),
                    current_user_id,
                },
                Some(db),
            )
            .await?;
            (obj_id, new_file_path)
        };

        // 新增对象引用
        let obj_ref_ro = OssObjRefSvc::add(
            OssObjRefAddDto {
                id: None,
                name: Some(file_name.to_string()),
                bucket_id: Some(one_bucket.id),
                obj_id: Some(obj_id.to_string()),
                ext: Some(ext.to_string()),
                current_user_id,
            },
            Some(db),
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

        Ok(obj_ref_ro.msg("上传成功".to_string()))
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
        db: Option<&DatabaseConnection>,
    ) -> Result<(String, u64, u64, Vec<u8>, Option<u64>, Option<u64>), SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
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
}
