use crate::settings::SETTINGS;
use log::info;
use sqlx::any::install_default_drivers;
use sqlx::AnyPool;

pub async fn migrate() -> Result<(), sqlx::Error> {
    info!("migrating database...");
    install_default_drivers();
    let db_url = SETTINGS.get().unwrap().db.url.as_str();
    let pool = AnyPool::connect(db_url).await?;
    Ok(sqlx::migrate!().run(&pool).await?)
}
