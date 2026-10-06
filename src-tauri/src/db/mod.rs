use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect("sqlite://flowdodb.sqlite?mode=rwc").await
}
