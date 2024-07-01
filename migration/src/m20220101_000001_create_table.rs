use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"
        DROP SCHEMA IF EXISTS finance;
            "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;
        let db = manager.get_connection();
        let sql = r#"
        CREATE SCHEMA finance;
            "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;
        let db = manager.get_connection();
        let sql = r#"
        CREATE EXTENSION pgcrypto;
            "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
