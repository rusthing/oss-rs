use crate::svc::OssObjRefSvc;
use crate::svc::OssObjSvc;
use robotech::macros::svc;

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
    #[log_call]
    pub async fn del_cascade<C>(
        id: u64,
        #[skip_log] db: Option<&C>,
    ) -> Result<Ro<OssBucketVo>, SvcError>
    where
        C: ConnectionTrait,
    {
        OssObjRefSvc::del_by_bucket_id(id, Some(db)).await?;
        OssObjSvc::delete_orphaned(Some(db)).await?;
        let ro = Self::del_by_id(id, Some(db)).await?;
        Ok(ro)
    }
}
