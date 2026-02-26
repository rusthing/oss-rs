use log::debug;
use robotech::db_conn::DbConfig;
use robotech_macros::log_call;
use sqlx::any::install_default_drivers;
use sqlx::AnyPool;

#[log_call]
pub async fn migrate(db: DbConfig) -> Result<(), sqlx::Error> {
    debug!("migrating database...");
    install_default_drivers();
    let db_url = db.url.as_str();
    let pool = AnyPool::connect(db_url).await?;
    Ok(sqlx::migrate!().run(&pool).await?)
}
