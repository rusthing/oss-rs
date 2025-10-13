use crate::base::svc::svc_error::{handle_db_err_to_svc_error, SvcError};
use crate::dao::oss_obj_dao::{OssObjDao, UNIQUE_FIELD_HASHMAP};
use crate::db::DB_CONN;
use crate::model::oss_obj::ActiveModel;
use crate::ro::ro::Ro;
use crate::to::oss_obj::{OssObjAddTo, OssObjModifyTo, OssObjSaveTo};
use crate::vo::oss_obj::OssObjVo;
use log::warn;
use sea_orm::DatabaseConnection;

pub struct OssObjSvc;

impl OssObjSvc {
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
        add_to: OssObjAddTo,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let active_model: ActiveModel = add_to.into();
        let one = OssObjDao::insert(active_model, db)
            .await
            .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELD_HASHMAP))?;
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
        modify_to: OssObjModifyTo,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let id = modify_to.id.clone().unwrap().parse::<u64>().unwrap();
        let active_model: ActiveModel = modify_to.into();
        OssObjDao::update(active_model, db)
            .await
            .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELD_HASHMAP))?;
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
        save_to: OssObjSaveTo,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjVo>, SvcError> {
        if save_to.id.clone().is_some() {
            Self::modify(save_to.into(), db).await
        } else {
            Self::add(save_to.into(), db).await
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
    /// * `Ok(Ro<()>)` - 删除成功，返回封装了成功消息的Ro对象
    /// * `Err(SvcError)` - 删除失败，可能因为记录不存在或其他数据库错误
    pub async fn del(
        id: u64,
        current_user_id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<()>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let del_model = Self::get_by_id(id, Some(db)).await?.get_extra().unwrap();
        warn!(
            "ID为<{}>的用户将删除oss_obj中的记录: {:?}",
            current_user_id, del_model
        );
        OssObjDao::delete(
            ActiveModel {
                id: sea_orm::ActiveValue::Set(id as i64),
                ..Default::default()
            },
            db,
        )
        .await
        .map_err(|e| handle_db_err_to_svc_error(e, &UNIQUE_FIELD_HASHMAP))?;
        Ok(Ro::success("删除成功".to_string()))
    }

    /// # 根据id获取记录信息
    ///
    /// 通过提供的ID从数据库中查询相应的记录，如果找到则返回封装在Ro中的Vo对象，否则返回NotFound错误
    ///
    /// ## 参数
    /// * `id` - 要查询的桶的ID
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<OssObjVo>)` - 查询成功，返回封装在Ro中的OssObjVo对象
    /// * `Err(SvcError)` - 查询失败，可能是因为记录不存在或其他数据库错误
    pub async fn get_by_id(
        id: u64,
        db: Option<&DatabaseConnection>,
    ) -> Result<Ro<OssObjVo>, SvcError> {
        let db = db.unwrap_or_else(|| DB_CONN.get().unwrap());
        let one = OssObjDao::get_by_id(id as i64, db).await?;
        Ok(Ro::success("查询成功".to_string()).extra(match one {
            Some(one) => Some(OssObjVo::from(one)),
            _ => return Err(SvcError::NotFound(format!("id: {}", id))),
        }))
    }
}
