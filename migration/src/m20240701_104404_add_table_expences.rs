use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"create table finance.expense
            (
                id           uuid default public.gen_random_uuid() not null primary key,
                created_at    timestamp with time zone default now() not null,
                value_sum numeric not null default 0
            );"#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("finance"), Alias::new("expense")))
                    .to_owned(),
            )
            .await
    }
}
