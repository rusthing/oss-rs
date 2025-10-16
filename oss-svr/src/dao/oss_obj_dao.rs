use crate::id_worker::ID_WORKER;
use crate::utils::time_utils::get_current_timestamp;
use once_cell::sync::Lazy;
use crate::model::oss_obj::{ActiveModel, Column, Entity, Model};
use crate::model::oss_obj_ref::{Column as OssObjRefColumn, Entity as OssObjRefEntity};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait,
    QueryFilter, QuerySelect, QueryTrait,
};
use std::collections::HashMap;

/// # 存储unique字段的HashMap
///
/// 在捕获到数据库重复键异常时，提取字段名称时可据此获取到字段的中文意义，方便提示给用户
pub static UNIQUE_FIELDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("path", "对象路径"),
        ("size, hash", "对象大小与Hash"),
        ("url", "对象URL"),
    ])
});

pub struct OssObjDao;

impl OssObjDao {
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

    /// # 获取孤立没有关联对象引用的记录
    ///
    /// 此函数负责获取那些在 `oss_obj_ref` 表中没有关联记录的 `oss_obj` 记录。
    /// 这有助于清理孤立的数据，释放存储空间。
    ///
    /// ## 参数
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回查询到的记录列表
    pub async fn find_orphaned<C>(db: &C) -> Result<Vec<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        // 使用子查询删除没有关联记录的oss_obj记录
        Entity::find()
            .filter(
                Column::Id.not_in_subquery(
                    OssObjRefEntity::find()
                        .select_only()
                        .column(OssObjRefColumn::ObjId)
                        .into_query(),
                ),
            )
            .all(db)
            .await
    }

    /// # 根据ID查询相应记录
    ///
    /// 此函数负责根据提供的ID从数据库中查询对应的记录
    ///
    /// ## 参数
    /// * `id` - 要查询的记录的ID
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回查询到的完整 Model 实例（如果存在），如果查询失败则返回相应的错误信息
    pub async fn get_by_id<C>(id: i64, db: &C) -> Result<Option<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Entity::find_by_id(id).one(db).await
    }

    /// # 根据哈希值和大小查询记录
    ///
    /// 此函数负责根据提供的哈希值和文件大小从数据库中查询对应的记录。
    /// 这通常用于检测是否已存在具有相同内容的文件，以避免重复存储。
    ///
    /// ## 参数
    /// * `hash` - 文件的哈希值（通常是MD5或SHA256等摘要算法的结果）
    /// * `size` - 文件的大小（以字节为单位）
    /// * `db` - 数据库连接 trait 对象
    ///
    /// ## 返回值
    /// 返回查询到的完整 Model 实例（如果存在），如果查询失败则返回相应的错误信息
    pub async fn get_by_hash_and_size<C>(
        hash: &str,
        size: i64,
        db: &C,
    ) -> Result<Option<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Entity::find()
            .filter(Column::Hash.eq(hash))
            .filter(Column::Size.eq(size))
            .one(db)
            .await
    }
}
