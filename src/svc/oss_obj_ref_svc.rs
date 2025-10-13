use crate::dao::oss_obj_ref_dao::OssObjRefDao;
use crate::db::DB_CONN;
use crate::ro::ro::Ro;
use crate::utils::svc_utils::SvcError;
use crate::vo::oss_obj_ref::OssObjRefVo;

/// 根据id获取对象信息
pub async fn get_by_id(obj_ref_id: u64) -> Result<Ro<OssObjRefVo>, SvcError> {
    let db = DB_CONN.get().unwrap();
    let one = OssObjRefDao::get_by_id(obj_ref_id as i64, db).await?;
    Ok(Ro::success("查询成功".to_string()).extra(match one {
        Some(one) => Some(OssObjRefVo::from(one)),
        _ => return Err(SvcError::NotFound(format!("id: {}", obj_ref_id))),
    }))
}
