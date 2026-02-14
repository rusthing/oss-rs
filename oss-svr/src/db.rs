use log::debug;
use robotech::db_conn::DbConfig;
use sqlx::AnyPool;
use sqlx::any::install_default_drivers;
use tracing::instrument;

#[instrument(level = "debug", err)]
pub async fn migrate(db: DbConfig) -> Result<(), sqlx::Error> {
    debug!("migrating database...");
    install_default_drivers();
    let db_url = db.url.as_str();
    let pool = AnyPool::connect(db_url).await?;
    Ok(sqlx::migrate!().run(&pool).await?)
}
