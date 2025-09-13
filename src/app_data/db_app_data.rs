use sea_orm::DatabaseConnection;

pub struct DbAppData {
    pub db: DatabaseConnection,
}
