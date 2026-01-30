use log::info;
use robotech::db::DbConfig;
use sqlx::AnyPool;
use sqlx::any::install_default_drivers;

pub async fn migrate(db: DbConfig) -> Result<(), sqlx::Error> {
    info!("migrating database...");
    install_default_drivers();
    let db_url = db.url.as_str();
    let pool = AnyPool::connect(db_url).await?;
    Ok(sqlx::migrate!().run(&pool).await?)
}
