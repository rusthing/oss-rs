use crate::dao::oss_obj_dao::OssObjDao;
use crate::dto::oss_obj_dto::{OssObjAddDto, OssObjModifyDto, OssObjSaveDto};
use crate::model::oss_obj::ActiveModel;
use crate::model::oss_obj::Model;
use crate::vo::oss_obj_vo::OssObjVo;
use log::warn;
use robotech::dao::begin_transaction;
use robotech::ro::Ro;
use robotech::svc::SvcError;
use robotech_macros::{db_unwrap, svc};
use sea_orm::{ActiveValue, ConnectionTrait};
use std::fs;

#[svc]
pub struct OssObjSvc;

impl OssObjSvc {
    /// # 删除记录及文件
    ///
    /// 根据提供的ID删除数据库中的相应记录，删除完成后会删除对象对应的文件，如果文件删除不成功则会回滚
    ///
    /// ## 参数
    /// * `id` - 要删除的记录的ID
    /// * `current_user_id` - 当前用户ID，用于记录操作
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 删除成功，返回封装了Vo的Ro对象
    /// * `Err(SvcError)` - 删除失败，可能因为记录不存在或其他数据库错误
    #[db_unwrap(transaction_required)]
    pub async fn del_with_file<C>(
        id: u64,
        current_user_id: u64,
        db: Option<&C>,
    ) -> Result<Ro<Model>, SvcError>
    where
        C: ConnectionTrait,
    {
        let ro = Self::del(id, current_user_id, Some(db)).await?;
        if let Some(extra) = ro.extra.clone() {
            // 删除文件
            fs::remove_file(extra.path)?;
        }
        Ok(ro)
    }

    /// # 删除孤立数据
    ///
    /// 删除那些在 `oss_obj_ref` 表中没有关联记录的 `oss_obj` 记录。
    /// 这有助于清理孤立的数据，释放存储空间。
    ///
    /// ## 参数
    /// * `current_user_id` - 当前用户ID，用于记录操作
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<String>)` - 删除成功，返回删除记录数
    /// * `Err(SvcError)` - 删除失败，可能因为数据库错误
    #[db_unwrap(transaction_required)]
    pub async fn delete_orphaned<C>(
        current_user_id: u64,
        db: Option<&C>,
    ) -> Result<Ro<String>, SvcError>
    where
        C: ConnectionTrait,
    {
        warn!(
            "ID为<{}>的用户将删除oss_obj中孤立无对象引用的记录",
            current_user_id
        );

        let result = OssObjDao::find_orphaned(db).await?;
        for item in result.into_iter() {
            Self::del_with_file(item.id as u64, current_user_id, Some(db)).await?;
        }

        Ok(Ro::success("删除孤立数据成功".to_string()))
    }

    /// # 根据哈希值和大小获取记录信息
    ///
    /// 通过提供的哈希值和文件大小从数据库中查询相应的记录，如果找到则返回封装了Vo的Ro对象，否则返回对象的extra为None
    ///
    /// ## 参数
    /// * `hash` - 文件的哈希值
    /// * `size` - 文件的大小
    /// * `db` - 数据库连接，如果未提供则使用全局数据库连接
    ///
    /// ## 返回值
    /// * `Ok(Ro<Vo>)` - 查询成功，如果记录存在，返回封装了Vo的Ro对象，如果不存在则返回对象的extra为None
    /// * `Err(SvcError)` - 查询失败，可能是数据库错误
    #[db_unwrap]
    pub async fn get_by_hash_and_size<C>(
        hash: &str,
        size: u64,
        db: Option<&C>,
    ) -> Result<Ro<OssObjVo>, SvcError>
    where
        C: ConnectionTrait,
    {
        let one = OssObjDao::get_by_hash_and_size(hash, size, db).await?;
        Ok(Ro::success("查询成功".to_string()).extra(one.map(|value| OssObjVo::from(value))))
    }
}
