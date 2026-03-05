use crate::dao::oss_bucket_dao::OssBucketDao;
use crate::dto::oss_bucket_dto::{OssBucketAddDto, OssBucketModifyDto, OssBucketSaveDto};
use crate::model::oss_bucket::ActiveModel;
use crate::model::oss_bucket::Model;
use crate::svc::oss_obj_ref_svc::OssObjRefSvc;
use crate::svc::oss_obj_svc::OssObjSvc;
use crate::vo::oss_bucket_vo::OssBucketVo;
use log::warn;
use robotech::dao::begin_transaction;
use robotech::ro::Ro;
use robotech::svc::SvcError;
use robotech_macros::{db_unwrap, svc};
use sea_orm::ConnectionTrait;

#[svc]
pub struct OssBucketSvc;

impl OssBucketSvc {
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
    #[db_unwrap(transaction_required)]
    pub async fn del_cascade<C>(
        id: u64,
        current_user_id: u64,
        db: Option<&C>,
    ) -> Result<Ro<Model>, SvcError>
    where
        C: ConnectionTrait,
    {
        OssObjRefSvc::del_by_bucket_id(id, current_user_id, Some(db)).await?;
        OssObjSvc::delete_orphaned(current_user_id, Some(db)).await?;
        let ro = Self::del(id, current_user_id, Some(db)).await?;
        Ok(ro)
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
    #[db_unwrap]
    pub async fn get_by_name<C>(name: &str, db: Option<&C>) -> Result<Ro<OssBucketVo>, SvcError>
    where
        C: ConnectionTrait,
    {
        let one = OssBucketDao::get_by_name(name, db).await?;
        Ok(Ro::success("查询成功".to_string()).extra(one.map(|value| OssBucketVo::from(value))))
    }
}
