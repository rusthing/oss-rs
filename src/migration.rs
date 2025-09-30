use crate::settings::SETTINGS;
use sqlx::PgPool;

pub async fn migrate() -> Result<(), sqlx::Error> {
    let db_url = SETTINGS.get().unwrap().clone().db.url;
    let pool = PgPool::connect(db_url.as_str()).await?;
    Ok(sqlx::migrate!().run(&pool).await?)
}
