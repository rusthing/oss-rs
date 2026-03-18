use crate::app::{get_app_config, AppConfig, OssConfig};
use crate::dao::OssObjRefDao;
use crate::dto::{OssObjAddDto, OssObjModifyDto};
use crate::dto::{OssObjRefAddDto, OssObjRefModifyDto};
use crate::svc::OssBucketSvc;
use crate::svc::OssObjRefSvc;
use crate::svc::OssObjSvc;
use crate::vo::OssObjRefVo;
use anyhow::anyhow;
use axum::body::Body;
use axum::extract::multipart::Field;
use axum::extract::Multipart;
use axum::http::{header, HeaderMap, HeaderValue};
use chrono::{Local, TimeZone};
use idworker::get_id_worker;
use log::{debug, info, trace, warn};
use robotech::dao::begin_transaction;
use robotech::env::{EnvError, APP_ENV};
use robotech::ro::Ro;
use robotech::svc::SvcError;
use robotech_macros::db_unwrap;
use sea_orm::ConnectionTrait;
use sha2::Digest;
use std::io::SeekFrom;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio_util::io::ReaderStream;
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
        // 获取存储桶
        let one_bucket = OssBucketSvc::get_by_name(bucket, Some(db)).await?;
        let one_bucket = match one_bucket.extra {
            Some(bucket) => bucket,
            None => return Ok(Ro::warn(format!("未找到存储桶<{}>", bucket))),
        };

        let mut hash_provided = None;
        let mut file_size_provided = None;
        // XXX 注意: 前端上传文件时，file参数必须放在最后
        while let Some(field) = multipart.next_field().await? {
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
                        .ok_or_else(|| validator::ValidationError::new("上传文件没有文件名"))?;

                    // 根据hash和size判断，如果对象已存在，则直接返回对象信息
                    let obj_vo = if let (Some(hash_provided), Some(file_size_provided)) =
                        (&hash_provided, &file_size_provided)
                    {
                        OssObjSvc::get_by_hash_and_size(
                            &hash_provided,
                            &file_size_provided,
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
                        info!("对象已存在，直接返回对象信息");
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
                        fs::create_dir_all(&storage_dir).await?;
                        let new_file_path = storage_dir
                            .join(obj_id.to_string())
                            .as_path()
                            .to_str()
                            .ok_or_else(|| {
                                std::io::Error::new(std::io::ErrorKind::Other, "Invalid file path")
                            })?
                            .to_string();

                        // 新增对象
                        let is_completed = false;
                        let oss_obj_add_dto = OssObjAddDto::builder()
                            .id(obj_id)
                            .path(new_file_path.to_string())
                            .is_completed(is_completed)
                            .current_user_id(current_user_id)
                            .build();

                        debug!("新增对象: {:?}", oss_obj_add_dto);
                        let add_ro = OssObjSvc::add(oss_obj_add_dto, Some(db)).await?;
                        if let Some(obj_vo) = add_ro.extra {
                            (false, obj_vo.id, new_file_path)
                        } else {
                            return Err(SvcError::Runtime(anyhow!("新增对象失败")));
                        }
                    };

                    // 新增对象引用
                    let obj_ref_id = get_id_worker()?.next_id()?;
                    let (obj_ref_name, preview_url) = if Self::is_previewable(&ext)
                        && let Some(ext) = &ext
                    {
                        let obj_ref_name = format!("{}.{}", obj_ref_id, ext);
                        let preview_url = Some(format!("/oss/file/preview/{}", obj_ref_name));
                        (obj_ref_name, preview_url)
                    } else {
                        let obj_ref_name = obj_ref_id.to_string();
                        (obj_ref_name, None)
                    };
                    let download_url = format!("/oss/file/download/{}", obj_ref_name);
                    let oss_obj_ref_add_dto = OssObjRefAddDto::builder()
                        .id(obj_ref_id)
                        .name(file_name.to_string())
                        .bucket_id(one_bucket.id)
                        .obj_id(obj_id)
                        .ext(ext)
                        .download_url(download_url)
                        .preview_url(preview_url)
                        .current_user_id(current_user_id)
                        .build();
                    debug!("新增对象引用: {:?}", oss_obj_ref_add_dto);
                    let obj_ref_ro = OssObjRefSvc::add(oss_obj_ref_add_dto, Some(db)).await?;

                    if !obj_exists {
                        let (file_size_computed, hash_computed) = Self::receive_and_write(
                            &hash_provided,
                            &file_size_provided,
                            field,
                            &new_file_path,
                        )
                        .await?;

                        // 写完文件时最后再检查一次文件大小和hash是否已经存在
                        let oss_obj_vo = OssObjSvc::get_by_hash_and_size(
                            &hash_computed,
                            &file_size_computed,
                            Some(db),
                        )
                        .await?
                        .extra;
                        if let Some(oss_obj_vo) = oss_obj_vo {
                            warn!(
                                "在上传完成后发现文件已存在，删除上传文件，引用的对象指向已存在的对象"
                            );
                            fs::remove_file(new_file_path).await?;
                            OssObjRefSvc::modify(
                                OssObjRefModifyDto::builder()
                                    .id(obj_ref_id)
                                    .obj_id(oss_obj_vo.id)
                                    .current_user_id(current_user_id)
                                    .build(),
                                Some(db),
                            )
                            .await?;
                        } else {
                            // 文件已上传完成，修改对象信息的hash、文件大小、是否完成
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
                    }
                    return Ok(obj_ref_ro.message("上传成功".to_string()));
                }
                _ => {}
            }
        }
        Err(validator::ValidationError::new("上传文件为空"))?
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
        headers: HeaderMap,
        obj_ref_id: u64,
        mut ext: Option<String>,
        db: Option<&C>,
    ) -> Result<
        (
            String,
            Option<String>,
            u64,
            u64,
            Body,
            Option<u64>,
            Option<u64>,
        ),
        SvcError,
    >
    where
        C: ConnectionTrait,
    {
        // 解析Range头：格式为 "bytes=start-end" 或 "bytes=start-"
        let (start, mut end) = match headers.get(header::RANGE) {
            Some(range) => Self::parse_range(range)?,
            // 如果没有Range头，则返回整个文件
            None => (None, None),
        };

        let one = OssObjRefDao::get_by_id_also_related(obj_ref_id, db).await?;
        let (obj_ref_model, _, obj_model) =
            one.ok_or(SvcError::NotFound(format!("id: {}", obj_ref_id)))?;
        // 如果有扩展名，扩展名不对也不行
        if &ext != &obj_ref_model.ext {
            return Err(SvcError::NotFound(format!("id: {}", obj_ref_id)));
        }
        ext = obj_ref_model.ext;

        // 读取文件指定范围内容
        let mut file = File::open(&obj_model.path).await?;
        let file_size = file.metadata().await?.len();
        let (chunk_size, body) = if let Some(start_pos) = start {
            if start_pos > file_size - 1 {
                Err(validator::ValidationError::new("起始位置超过了文件末尾"))?;
            }
            // 如果没有指定结束位置，则默认为文件末尾
            let end_pos = end.unwrap_or({
                let end_pos = file_size - 1;
                end = Some(end_pos);
                end_pos
            });
            if end_pos > file_size - 1 {
                Err(validator::ValidationError::new("结束位置超过了文件末尾"))?;
            }

            let chunk_size = end_pos - start_pos + 1;
            file.seek(SeekFrom::Start(start_pos)).await?;

            // 用 Take 限制只读 chunk_size 字节，再包成流
            let limited = file.take(chunk_size);
            let buffer_size = get_app_config()?.oss.download_buffer_size.as_u64() as usize;
            let stream = ReaderStream::with_capacity(limited, buffer_size);
            let body = Body::from_stream(stream);
            (chunk_size, body)
        } else {
            // 将文件转为异步流
            let read_stream = ReaderStream::new(file);
            let body = Body::from_stream(read_stream);
            (file_size, body)
        };

        Ok((
            obj_ref_model.name,
            ext,
            file_size,
            chunk_size,
            body,
            start,
            end,
        ))
    }

    async fn receive_and_write(
        hash_provided: &Option<String>,
        file_size_provided: &Option<u64>,
        mut field: Field<'_>,
        new_file_path: &str,
    ) -> Result<(u64, String), SvcError> {
        // 如果对象不存在，则开始接收 chunk
        let AppConfig { oss, .. } = get_app_config().expect("app config not found");
        let OssConfig {
            upload_file_limit_size,
            upload_buffer_size,
            ..
        } = oss;

        let buffer_size = upload_buffer_size.as_u64();
        let mut file_size_computed: u64 = 0;
        let mut file = File::create(new_file_path).await?;
        let mut hasher = sha2::Sha256::new();
        let mut buffer = Vec::with_capacity(buffer_size as usize);
        // 分块写入，而非 .bytes() 一次性读取
        while let Some(chunk) = field.chunk().await? {
            let chunk_size = chunk.len() as u64;
            file_size_computed += chunk_size;
            if file_size_computed > upload_file_limit_size.as_u64() {
                fs::remove_file(new_file_path).await?;
                return Err(SvcError::Runtime(anyhow!(
                    "上传文件大小超出限制: {upload_file_limit_size}"
                )));
            }
            hasher.update(&chunk);
            if chunk_size == buffer_size {
                trace!("切片大小 == 缓存大小，直接写入盘(没有用到缓存)");
                file.write_all(&chunk).await?;
            } else if chunk_size > buffer_size {
                trace!(
                    "切片大小 > 缓存大小，chunk按缓存大小分片处理，避免单次写入过大(这里也没有用到缓存)"
                );
                let mut chunk_start: usize = 0;
                while chunk_start < chunk_size as usize {
                    let chunk_end = (chunk_start + buffer_size as usize).min(chunk_size as usize);
                    let slice = &chunk[chunk_start..chunk_end];
                    file.write_all(slice).await?;
                    chunk_start = chunk_end;
                }
            } else {
                trace!("切片大小 < 缓存大小，攒够缓存满才写一次盘");
                let buffer_length = buffer.len() as u64; // 缓存实际大小
                let total_size = buffer_length + chunk_size; // 总大小 = 缓存大小 + 切片大小
                let chunk_remain = total_size - buffer_length; // 切片剩余大小
                // 如果切片剩余大小 <= 0，则将切片添加入缓存中，否则将缓存中的数据写入盘，并清空缓存，再将剩余切片写入缓存中
                if chunk_remain <= 0 {
                    buffer.extend_from_slice(&chunk);
                } else {
                    file.write_all(&buffer).await?;
                    buffer.clear();
                    buffer.extend_from_slice(&chunk);
                }
            }
        }
        // 写入剩余的缓存部分
        if !buffer.is_empty() {
            file.write_all(&buffer).await?;
        }

        if let Some(file_size_provided) = file_size_provided
            && file_size_provided != &file_size_computed
        {
            fs::remove_file(new_file_path).await?;
            return Err(SvcError::Runtime(anyhow!(
                "上传文件大小错误，请重新上传: {file_size_provided}->{file_size_computed}"
            )));
        }
        let hash_computed = format!("{:x}", hasher.finalize());
        if let Some(hash_provided) = hash_provided.clone()
            && hash_provided != hash_computed
        {
            fs::remove_file(new_file_path).await?;
            return Err(SvcError::Runtime(anyhow!(
                "上传文件内容校验错误，请重新上传: {hash_provided}->{hash_computed}"
            )));
        }
        Ok((file_size_computed, hash_computed))
    }

    fn parse_range(range: &HeaderValue) -> Result<(Option<u64>, Option<u64>), SvcError> {
        // "bytes=500-999"  → (500, 999)
        // "bytes=500-"     → (500, total-1)
        let range = range
            .to_str()
            .map_err(|_| validator::ValidationError::new("无效的Range格式"))?;
        let range = range
            .strip_prefix("bytes=")
            .ok_or_else(|| validator::ValidationError::new("无效的Range格式"))?;

        let parts: Vec<&str> = range.split("-").collect();
        let start = Some(
            parts
                .get(0)
                .ok_or_else(|| validator::ValidationError::new("无效的Range格式"))?
                .parse::<u64>()
                .map_err(|_| validator::ValidationError::new("无效的Range格式"))?,
        );
        let end = if parts.len() > 1 && !parts[1].is_empty() {
            // len>1，【1]取法没问题
            Some(
                parts[1]
                    .parse::<u64>()
                    .map_err(|_| validator::ValidationError::new("无效的Range格式"))?,
            )
        } else {
            None
        };

        Ok((start, end))
    }

    fn is_previewable(ext: &Option<String>) -> bool {
        if let Some(ext) = ext {
            match ext.as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "pdf" | "txt" | "md" | "mp3"
                | "wav" | "ogg" | "aac" | "flac" | "mp4" => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn get_content_type_of_preview(ext: &Option<String>) -> Option<&'static str> {
        if let Some(ext) = ext {
            match ext.as_str() {
                "jpg" | "jpeg" => Some("image/jpeg"),
                "png" => Some("image/png"),
                "gif" => Some("image/gif"),
                "webp" => Some("image/webp"),
                "svg" => Some("image/svg+xml"),
                "pdf" => Some("application/pdf"),
                "txt" | "md" => Some("text/plain"),
                "mp3" | "wav" | "ogg" | "aac" | "flac" => Some("audio/mpeg"),
                "mp4" => Some("video/mp4"),
                _ => None,
            }
        } else {
            None
        }
    }
}
