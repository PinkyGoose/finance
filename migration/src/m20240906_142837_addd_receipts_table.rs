use sea_orm_migration::prelude::*;
use crate::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"create table finance.receipt
            (
                id           uuid default public.gen_random_uuid() not null primary key,
                start_date    timestamp with time zone default now() not null,
                stop_date timestamp with time zone default now() not null,
                sallary numeric not null default 0,
                user_id uuid,
                FOREIGN KEY (user_id)  REFERENCES security.user (id) ON DELETE CASCADE
            );"#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("finance"), Alias::new("receipt")))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
