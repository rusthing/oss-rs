use crate::dao::OssObjRefDao;
use crate::dto::{OssObjRefAddDto, OssObjRefModifyDto, OssObjRefSaveDto};
use crate::model::oss_obj_ref::ActiveModel;
use crate::svc::OssObjSvc;
use crate::vo::OssObjRefVo;
use robotech_macros::svc;

#[svc]
pub struct OssObjRefSvc;
impl OssObjRefSvc {
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
    #[db_unwrap(transaction_required)]
    pub async fn del_with_obj<C>(id: u64, db: Option<&C>) -> Result<Ro<OssObjRefVo>, SvcError>
    where
        C: ConnectionTrait,
    {
        let ro = Self::del(id, Some(db)).await?;
        if let Some(extra) = ro.extra.clone() {
            // 删除对象, 如果对象没有其他引用则会顺利删除，否则会失败
            OssObjSvc::del_with_file(extra.obj_id, Some(db)).await.ok();
        }
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
    #[db_unwrap(transaction_required)]
    pub async fn del_by_bucket_id<C>(bucket_id: u64, db: Option<&C>) -> Result<Ro<()>, SvcError>
    where
        C: ConnectionTrait,
    {
        OssObjRefDao::delete_by_bucket_id(bucket_id, db).await?;
        Ok(Ro::success("删除成功".to_string()))
    }
}
