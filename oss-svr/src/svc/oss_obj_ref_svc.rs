use crate::dao::oss_obj_ref_dao::{OssObjRefDao, UNIQUE_FIELDS};
use crate::dto::oss_obj_ref_dto::{OssObjRefAddDto, OssObjRefModifyDto, OssObjRefSaveDto};
use crate::model::oss_obj_ref::ActiveModel;
use crate::svc::oss_obj_svc::OssObjSvc;
use crate::vo::oss_obj_ref::OssObjRefVo;
use log::warn;
use robotech::db::DB_CONN;
use robotech::ro::Ro;
use robotech::svc::svc_error::{handle_db_err_to_svc_error, SvcError};
use sea_orm::DatabaseConnection;

pub struct OssObjRefSvc;
impl OssObjRefSvc {
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
        add_dto: OssObjRefAddDto,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjRefVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let active_model: ActiveModel = add_dto.into();
        let one = OssObjRefDao::insert(active_model, db)
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
        modify_dto: OssObjRefModifyDto,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjRefVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let id = modify_dto.id.clone().unwrap().parse::<u64>().unwrap();
        let active_model: ActiveModel = modify_dto.into();
        OssObjRefDao::update(active_model, db)
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
        save_dto: OssObjRefSaveDto,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjRefVo>, SvcError> {
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
    ) -> Result<Ro<OssObjRefVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let del_model = Self::get_by_id(id, Some(db))
            .await?
            .get_extra()
            .ok_or(SvcError::NotFound(id.to_string()))?;
        warn!(
            "ID为<{}>的用户将删除oss_obj_ref中的记录: {:?}",
            current_user_id,
            del_model.clone()
        );
        OssObjRefDao::delete(
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

    /// # 删除对象引用及对象
    ///
    /// 根据提供的ID删除数据库中的相应记录，并删除对应的对象，如果对象没有其他引用则会顺利删除，否则不做任何事
    ///
    /// ## 参数
    /// * `id` - 要删除的记录的ID
    /// * `current_user_id` - 当前用户ID
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 删除成功，返回封装了Vo的Ro对象
    pub async fn del_with_obj(
        id: u64,
        current_user_id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjRefVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let ro = Self::del(id, current_user_id, Some(db)).await?;
        // 删除对象, 如果对象没有其他引用则会顺利删除，否则会失败
        let obj_id = ro.extra.clone().unwrap().oss_obj.id.parse::<u64>().unwrap();
        OssObjSvc::del_with_file(obj_id, current_user_id, Some(db))
            .await
            .ok();
        Ok(ro)
    }

    /// # 根据bucket_id删除对象引用记录
    ///
    /// 根据提供的bucket_id从数据库中删除相应的记录
    ///
    /// ## 参数
    /// * `bucket_id` - 要删除符合bucket_id为此值的所有记录
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vec<OssObjRefVo>>)` - 删除成功，返回封装了Vo的Ro对象
    /// * `Err(SvcError)` - 删除失败，可能是数据库错误
    pub async fn del_by_bucket_id(
        bucket_id: u64,
        current_user_id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<()>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        warn!(
            "ID为<{}>的用户将删除oss_obj_ref中bucket_id={}的记录",
            current_user_id, bucket_id
        );
        OssObjRefDao::delete_by_bucket_id(bucket_id as i64, db).await?;
        Ok(Ro::success("删除成功".to_string()))
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
    ) -> Result<Ro<OssObjRefVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let one = OssObjRefDao::get_by_id(id as i64, db).await?;
        Ok(Ro::success("查询成功".to_string()).extra(one.map(|value| OssObjRefVo::from(value))))
    }
}
