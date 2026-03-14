use crate::app::{get_app_config, AppConfig, OssConfig};
use crate::dao::OssObjRefDao;
use crate::dto::OssObjRefAddDto;
use crate::dto::{OssObjAddDto, OssObjModifyDto};
use crate::svc::OssBucketSvc;
use crate::svc::OssObjRefSvc;
use crate::svc::OssObjSvc;
use crate::vo::OssObjRefVo;
use anyhow::anyhow;
use axum::extract::Multipart;
use bytesize::ByteSize;
use chrono::{Local, TimeZone};
use idworker::get_id_worker;
use log::debug;
use robotech::dao::begin_transaction;
use robotech::env::{EnvError, APP_ENV};
use robotech::ro::Ro;
use robotech::svc::SvcError;
use robotech_macros::db_unwrap;
use sea_orm::ConnectionTrait;
use sha2::Digest;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use wheel_rs::file_utils::get_file_ext;
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
    #[db_unwrap(transaction_required)]
    pub async fn upload<C>(
        bucket: &str,
        mut multipart: Multipart,
        current_user_id: u64,
        db: Option<&C>,
    ) -> Result<Ro<OssObjRefVo>, SvcError>
    where
        C: ConnectionTrait,
    {
        let AppConfig { oss, .. } = get_app_config().expect("app config not found");
        let OssConfig {
            upload_file_limit_size,
            ..
        } = oss;

        // 获取存储桶
        let one_bucket = OssBucketSvc::get_by_name(bucket, Some(db)).await?;
        let one_bucket = match one_bucket.extra {
            Some(bucket) => bucket,
            None => return Ok(Ro::warn(format!("未找到存储桶<{}>", bucket))),
        };

        let mut hash_provided = None;
        let mut file_size_provided = None;
        // XXX 注意: 前端上传文件时，file参数必须放在最后
        while let Some(mut field) = multipart.next_field().await? {
            match field.name() {
                Some("hash") => {
                    hash_provided = Some(field.text().await?);
                }
                Some("size") => {
                    file_size_provided = Some(
                        field
                            .text()
                            .await?
                            .parse::<u64>()
                            .map_err(|_| validator::ValidationError::new("文件大小格式错误"))?,
                    );
                }
                Some("file") => {
                    let file_name = field
                        .file_name()
                        .ok_or_else(|| validator::ValidationError::new("上传文件必须包含文件名"))?;

                    // 根据hash和size判断，如果对象已存在，则直接返回对象信息
                    let obj_vo = if let Some(hash_provided) = hash_provided.clone()
                        && let Some(file_size_provided) = file_size_provided.clone()
                    {
                        OssObjSvc::get_by_hash_and_size(
                            &hash_provided,
                            file_size_provided,
                            Some(db),
                        )
                        .await?
                        .extra
                    } else {
                        None
                    };

                    let now = get_current_timestamp()?;
                    let ext = get_file_ext(file_name);

                    // 判断对象是否存在
                    let (obj_exists, obj_id, new_file_path) = if let Some(obj_vo) = obj_vo {
                        // 如果已经上传过该文件，则直接返回之前的对象ID和存放路径
                        (true, obj_vo.id, obj_vo.path)
                    } else {
                        // 如果未上传过该文件，则新增对象，并返回新对象ID和新文件的存放路径
                        let obj_id = get_id_worker()?.next_id()?;
                        // 根据当前时间，创建yyyy/MM/dd/HH的目录，并将文件存入此目录中
                        let datetime = Local.timestamp_opt((now / 1000) as i64, 0).unwrap();
                        let oss_config = get_app_config()?.oss;

                        let date_path = datetime.format(&oss_config.file_dir_format).to_string();

                        let storage_dir = APP_ENV
                            .get()
                            .ok_or(EnvError::GetAppEnv())?
                            .app_dir
                            .join(&oss_config.file_root_dir)
                            .join(bucket.to_string())
                            .join(&date_path);
                        fs::create_dir_all(&storage_dir)?;
                        let new_file_path = storage_dir
                            .join(obj_id.to_string())
                            .as_path()
                            .to_str()
                            .unwrap()
                            .to_string();

                        // 新增对象
                        let is_completed = false;
                        let oss_obj_add_dto = OssObjAddDto::builder()
                            .id(obj_id)
                            // .hash(hash_provided.to_string())
                            // .size(file_size_provided)
                            .path(new_file_path.to_string())
                            .is_completed(is_completed)
                            .current_user_id(current_user_id)
                            .build();

                        debug!("新增对象: {:?}", oss_obj_add_dto);
                        let add_ro = OssObjSvc::add(oss_obj_add_dto, Some(db)).await?;
                        if let Some(obj_vo) = add_ro.extra {
                            (false, obj_vo.id, new_file_path)
                        } else {
                            return Err(SvcError::Runtime(anyhow!("新增对象失败".to_string())));
                        }
                    };

                    // 新增对象引用
                    let obj_ref_id = get_id_worker()?.next_id()?;
                    let obj_ref_name = format!("{}.{}", obj_ref_id, ext);
                    let obj_ref_url = format!("/oss/file/preview/{}", obj_ref_name);
                    let oss_obj_ref_add_dto = OssObjRefAddDto::builder()
                        .id(obj_ref_id)
                        .name(file_name.to_string())
                        .bucket_id(one_bucket.id)
                        .obj_id(obj_id)
                        .ext(ext.to_string())
                        .url(obj_ref_url)
                        .current_user_id(current_user_id)
                        .build();
                    debug!("新增对象引用: {:?}", oss_obj_ref_add_dto);
                    let obj_ref_ro = OssObjRefSvc::add(oss_obj_ref_add_dto, Some(db)).await?;

                    if !obj_exists {
                        // 如果对象不存在，则开始接收 chunk
                        let mut file_size_computed: u64 = 0;
                        let mut file = File::create(new_file_path.clone())?;
                        let mut hasher = sha2::Sha256::new();
                        // 分块写入，而非 .bytes() 一次性读取
                        while let Some(chunk) = field.chunk().await? {
                            file_size_computed += chunk.len() as u64;
                            if file_size_computed > upload_file_limit_size.as_u64() {
                                fs::remove_file(new_file_path)?;
                                return Err(SvcError::Runtime(anyhow!(
                                    "上传文件大小超出限制: {upload_file_limit_size}"
                                )));
                            }
                            hasher.update(&chunk);
                            file.write_all(&chunk)?;
                        }
                        if let Some(file_size_provided) = file_size_provided
                            && file_size_provided != file_size_computed
                        {
                            fs::remove_file(new_file_path)?;
                            return Err(SvcError::Runtime(anyhow!(
                                "上传文件大小错误，请重新上传: {file_size_provided}->{file_size_computed}"
                            )));
                        }
                        let hash_computed = format!("{:x}", hasher.finalize());
                        if let Some(hash_provided) = hash_provided.clone()
                            && hash_provided != hash_computed
                        {
                            fs::remove_file(new_file_path)?;
                            return Err(SvcError::Runtime(anyhow!(
                                "上传文件内容校验错误，请重新上传: {hash_provided}->{hash_computed}"
                            )));
                        }
                        let is_completed = true;
                        OssObjSvc::modify(
                            OssObjModifyDto::builder()
                                .id(obj_id)
                                .hash(Some(hash_computed))
                                .size(Some(file_size_computed))
                                .is_completed(is_completed)
                                .current_user_id(current_user_id)
                                .build(),
                            Some(db),
                        )
                        .await?;
                    }
                    return Ok(obj_ref_ro.message("上传成功".to_string()));
                }
                _ => {}
            }
        }
        Err(validator::ValidationError::new("上传文件必须包含文件"))?
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
    #[db_unwrap]
    pub async fn download<C>(
        obj_ref_id: u64,
        ext: String,
        start: Option<u64>,
        mut end: Option<u64>,
        db: Option<&C>,
    ) -> Result<(String, u64, u64, Vec<u8>, Option<u64>, Option<u64>), SvcError>
    where
        C: ConnectionTrait,
    {
        let one = OssObjRefDao::get_by_id_also_related(obj_ref_id, db).await?;
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
            let size = get_app_config()?.oss.download_buffer_size.as_u64();
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
