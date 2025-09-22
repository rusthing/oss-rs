use crate::model::oss_obj::{Entity, Model};
use crate::utils::time_utils::get_current_timestamp;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    EntityTrait, ExecResult, IntoActiveModel, QueryFilter, Statement,
};

/// 添加
pub async fn insert(db: &DatabaseTransaction, model: Model) -> Result<Model, DbErr> {
    let active_model = model.into_active_model();
    active_model.insert(db).await
}

/// 根据ID修改引用计数+1（原子更新）
pub async fn increment_ref_count_atomic(
    db: &DatabaseConnection,
    model: Model,
) -> Result<ExecResult, DbErr> {
    Ok(db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"UPDATE "oss_obj" SET "ref_count" = "ref_count" + 1, "update_timestamp" = $1 WHERE "id" = $2 and "update_timestamp" = $3"#,
        vec![
            (get_current_timestamp() as i64).into(),
            model.id.into(),
            model.update_timestamp.into()
        ],
    )).await?)
}

/// 根据ID修改引用计数-1（原子更新）
pub async fn decrement_ref_count_atomic(
    db: &DatabaseConnection,
    model: Model,
) -> Result<ExecResult, DbErr> {
    Ok(db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"UPDATE "oss_obj" SET "ref_count" = "ref_count" - 1, "update_timestamp" = $1 WHERE "id" = $2 and "update_timestamp" = $3"#,
        vec![
            (get_current_timestamp() as i64).into(),
            model.id.into(),
            model.update_timestamp.into()
        ],
    )).await?)
}

pub async fn delete(db: &DatabaseConnection, model: Model) -> Result<(), DbErr> {
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"DELETE FROM "oss_obj" WHERE "id" = $1 and "update_timestamp" = $2"#,
        vec![model.id.into(), model.update_timestamp.into()],
    ))
    .await?;
    Ok(())
}

/// 根据id查询
pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id as i64).one(db).await
}

/// 根据id查询
pub async fn get_by_hash_and_size(
    db: &DatabaseConnection,
    hash: &str,
    size: i64,
) -> Result<Option<Model>, DbErr> {
    Entity::find()
        .filter(crate::model::oss_obj::Column::Hash.eq(hash))
        .filter(crate::model::oss_obj::Column::Size.eq(size))
        .one(db)
        .await
}
