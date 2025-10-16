use crate::id_worker::ID_WORKER;
use crate::model::oss_obj_ref::{ActiveModel, Column, Entity, Model};
use crate::model::{oss_bucket, oss_obj};
use once_cell::sync::Lazy;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait,
    QueryFilter,
};
use std::collections::HashMap;
use wheel_rs::time_utils::get_current_timestamp;

/// 存储unique字段的HashMap
/// 在捕获到数据库重复键异常时，提取字段名称时可据此获取到字段的中文意义，方便提示给用户
pub static UNIQUE_FIELDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| HashMap::new());

pub struct OssObjRefDao;

impl OssObjRefDao {
    /// # 插入记录
    ///
    /// 此函数负责向数据库中插入一个新的记录。它会自动处理以下逻辑：
    /// - 如果记录 ID 未设置（默认值），则生成一个新的唯一 ID
    /// - 如果创建时间戳未设置，则设置当前时间为创建和更新时间
    /// - 将修改者 ID 设置为创建者 ID（因为是新建记录）
    ///
    /// ## 参数
    /// * `active_model` - 包含待插入数据的 ActiveModel 实例
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回插入后的完整 Model 实例，如果插入失败则返回相应的错误信息
    pub async fn insert<C>(mut active_model: ActiveModel, db: &C) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        // 当id为默认值(0)时生成ID
        if active_model.id == ActiveValue::NotSet {
            active_model.id = ActiveValue::set(ID_WORKER.get().unwrap().next_id() as i64);
        }
        // 当创建时间未设置时，设置创建时间和修改时间
        if active_model.create_timestamp == ActiveValue::NotSet {
            let now = ActiveValue::set(get_current_timestamp() as i64);
            active_model.create_timestamp = now.clone();
            active_model.update_timestamp = now;
        }
        // 添加时修改者就是创建者
        active_model.updator_id = active_model.creator_id.clone();
        // 执行数据库插入操作
        active_model.insert(db).await
    }

    /// # 更新记录
    ///
    /// 此函数负责更新数据库中的现有记录。它会自动处理以下逻辑：
    /// - 如果更新时间戳未设置，则设置当前时间为更新时间
    /// - 更新完成后，重新查询并返回更新后的完整记录
    ///
    /// ## 参数
    /// * `active_model` - 包含待更新数据的 ActiveModel 实例
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回更新后的完整 Model 实例，如果更新失败则返回相应的错误信息
    pub async fn update<C>(mut active_model: ActiveModel, db: &C) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        // 当修改时间未设置时，设置修改时间
        if active_model.update_timestamp == ActiveValue::NotSet {
            let now = ActiveValue::set(get_current_timestamp() as i64);
            active_model.update_timestamp = now;
        }
        // 执行数据库更新操作
        active_model.update(db).await
    }

    /// # 删除记录
    ///
    /// 此函数负责根据关键字段删除相应的记录
    ///
    /// ## 参数
    /// * `active_model` - 包含待删除数据的 ActiveModel 实例
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 如果删除成功则返回 Ok(())，如果删除失败则返回相应的错误信息
    pub async fn delete<C>(active_model: ActiveModel, db: &C) -> Result<DeleteResult, DbErr>
    where
        C: ConnectionTrait,
    {
        active_model.delete(db).await
    }

    pub async fn delete_by_bucket_id<C>(bucket_id: i64, db: &C) -> Result<DeleteResult, DbErr>
    where
        C: ConnectionTrait,
    {
        Entity::delete_many()
            .filter(Column::BucketId.eq(bucket_id))
            .exec(db)
            .await
    }

    /// # 根据ID查询记录
    ///
    /// 此函数通过给定的ID查询单条记录，并同时获取关联的存储桶和对象信息
    ///
    /// ## 参数
    /// * `id` - 要查询的记录的唯一标识符
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回一个包含主记录、关联存储桶和关联对象的元组的Option，如果查询失败则返回相应的错误信息
    /// 如果未找到匹配记录，则返回None
    pub async fn get_by_id<C>(
        id: i64,
        db: &C,
    ) -> Result<Option<(Model, oss_bucket::Model, oss_obj::Model)>, DbErr>
    where
        C: ConnectionTrait,
    {
        Entity::find_by_id(id)
            .find_also_related(oss_bucket::Entity)
            .find_also_related(oss_obj::Entity)
            .one(db)
            .await
            .map(|model_option| {
                model_option.map(|(model, bucket_option, obj_option)| {
                    (model, bucket_option.unwrap(), obj_option.unwrap())
                })
            })
    }
}
