use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"
        alter table finance.expense
            add user_id uuid,
            ADD FOREIGN KEY (user_id) references security.user(id)
            on delete cascade;
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"
        alter table finance.expense
            drop column user_id
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
