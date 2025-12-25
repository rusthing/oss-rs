use crate::dao::oss_bucket_dao::{OssBucketDao, UNIQUE_FIELDS};
use crate::dto::oss_bucket_dto::{OssBucketAddDto, OssBucketModifyDto, OssBucketSaveDto};
use crate::model::oss_bucket::ActiveModel;
use crate::svc::oss_obj_ref_svc::OssObjRefSvc;
use crate::svc::oss_obj_svc::OssObjSvc;
use crate::vo::oss_bucket_vo::OssBucketVo;
use log::warn;
use robotech::db::DB_CONN;
use robotech::ro::Ro;
use robotech::svc::svc_error::{handle_db_err_to_svc_error, SvcError};
use sea_orm::DatabaseConnection;

pub struct OssBucketSvc;

impl OssBucketSvc {
    /// # 添加新记录
    ///
    /// 将提供的AddTo对象转换为ActiveModel并插入到数据库中
    ///
    /// ## 参数
    /// * `add_to` - 包含要添加记录信息的传输对象
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 添加成功，返回封装了新增Vo的Ro对象
    /// * `Err(SvcError)` - 添加失败，可能是因为违反唯一约束或其他数据库错误
    pub async fn add(
        add_dto: OssBucketAddDto,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssBucketVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let active_model: ActiveModel = add_dto.into();
        let one = OssBucketDao::insert(active_model, db)
            .await
            .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELDS))?;
        Ok(Self::get_by_id(one.id as u64, Some(db))
            .await?
            .msg("添加成功".to_string()))
    }

    /// # 修改记录
    ///
    /// 根据提供的ModifyTo对象更新数据库中的相应记录
    ///
    /// ## 参数
    /// * `modify_to` - 包含要修改记录信息的传输对象，必须包含有效的ID
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 修改成功，返回封装了更新后Vo的Ro对象
    /// * `Err(SvcError)` - 修改失败，可能因为记录不存在、违反唯一约束或其他数据库错误
    pub async fn modify(
        modify_dto: OssBucketModifyDto,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssBucketVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let id = modify_dto.id.unwrap();
        let active_model: ActiveModel = modify_dto.into();
        OssBucketDao::update(active_model, db)
            .await
            .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELDS))?;
        Ok(Self::get_by_id(id, Some(db))
            .await?
            .msg("修改成功".to_string()))
    }

    /// # 保存记录
    ///
    /// 根据提供的SaveTo对象保存记录到数据库中。如果提供了ID，则更新现有记录；如果没有提供ID，则创建新记录
    ///
    /// ## 参数
    /// * `save_to` - 包含要保存记录信息的传输对象
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 保存成功，返回封装了Vo的Ro对象
    /// * `Err(SvcError)` - 保存失败，可能因为违反唯一约束、记录不存在或其他数据库错误
    pub async fn save(
        save_dto: OssBucketSaveDto,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssBucketVo>, SvcError> {
        if save_dto.id.clone().is_some() {
            Self::modify(save_dto.into(), db).await
        } else {
            Self::add(save_dto.into(), db).await
        }
    }

    /// # 删除记录
    ///
    /// 根据提供的ID删除数据库中的相应记录
    ///
    /// ## 参数
    /// * `id` - 要删除的记录的ID
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 删除成功，返回封装了Vo的Ro对象
    /// * `Err(SvcError)` - 删除失败，可能因为记录不存在或其他数据库错误
    pub async fn del(
        id: u64,
        current_user_id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssBucketVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let del_model = Self::get_by_id(id, Some(db))
            .await?
            .get_extra()
            .ok_or(SvcError::NotFound(id.to_string()))?;
        warn!(
            "ID为<{}>的用户将删除oss_bucket中的记录: {:?}",
            current_user_id,
            del_model.clone()
        );
        OssBucketDao::delete(
            ActiveModel {
                id: sea_orm::ActiveValue::Set(id as i64),
                ..Default::default()
            },
            db,
        )
        .await
        .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELDS))?;
        Ok(Ro::success("删除成功".to_string()).extra(Some(del_model)))
    }

    /// # 级联删除记录
    ///
    /// 根据提供的ID删除数据库中的相应记录，并级联删除相关联的数据
    ///
    /// ## 参数
    /// * `id` - 要删除的记录的ID
    /// * `current_user_id` - 当前操作用户ID
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 删除成功，返回封装了Vo的Ro对象
    /// * `Err(SvcError)` - 删除失败，可能因为记录不存在或其他数据库错误
    pub async fn del_cascade(
        id: u64,
        current_user_id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssBucketVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        OssObjRefSvc::del_by_bucket_id(id, current_user_id, Some(db)).await?;
        OssObjSvc::delete_orphaned(current_user_id, Some(db)).await?;
        let ro = Self::del(id, current_user_id, Some(db)).await?;
        Ok(ro)
    }

    /// # 根据id获取记录信息
    ///
    /// 通过提供的ID从数据库中查询相应的记录，如果找到则返回封装了Vo的Ro对象，否则返回对象的extra为None
    ///
    /// ## 参数
    /// * `id` - 要查询的桶的ID
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 查询成功，如果记录存在，返回封装了Vo的Ro对象，如果不存在则返回对象的extra为None
    /// * `Err(SvcError)` - 查询失败，可能是数据库错误
    pub async fn get_by_id(
        id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssBucketVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let one = OssBucketDao::get_by_id(id as i64, db).await?;
        Ok(Ro::success("查询成功".to_string()).extra(one.map(|value| OssBucketVo::from(value))))
    }

    /// # 根据名称获取记录信息
    ///
    /// 通过提供的名称从数据库中查询相应的记录，如果找到则返回封装了Vo的Ro对象，否则返回对象的extra为None
    ///
    /// ## 参数
    /// * `name` - 要查询的桶的名称
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 查询成功，如果记录存在，返回封装了Vo的Ro对象，如果不存在则返回对象的extra为None
    /// * `Err(SvcError)` - 查询失败，可能是数据库错误
    pub async fn get_by_name(
        name: &str,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssBucketVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let one = OssBucketDao::get_by_name(name, db).await?;
        Ok(Ro::success("查询成功".to_string()).extra(one.map(|value| OssBucketVo::from(value))))
    }
}
