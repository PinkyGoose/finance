use sea_orm_migration::prelude::*;

use crate::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"CREATE OR REPLACE VIEW finance.expense_with_receipt AS
WITH daily_sallary_per_job AS (
    SELECT
        e.id as id,
        e.user_id,
        e.created_at::date AS expense_date,
        e.value_sum AS expense_amount,
        COALESCE(r.sallary / EXTRACT(DAY FROM date_trunc('month', e.created_at) + INTERVAL '1 month' - INTERVAL '1 day'), 0) AS daily_sallary
    FROM
        finance.expense e
    LEFT JOIN
        finance.receipt r
        ON e.user_id = r.user_id
        AND e.created_at BETWEEN r.start_date AND COALESCE(r.stop_date, e.created_at) -- если stop_date NULL, считаем работу актуальной
)
SELECT
    id
    user_id,
    expense_date,
    expense_amount,
    SUM(daily_sallary) AS total_daily_sallary,
    SUM(daily_sallary) - expense_amount AS balance -- расчет баланса
FROM
    daily_sallary_per_job
GROUP BY
    user_id, expense_date, expense_amount, id;"#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        db.execute(stmt).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let sql = r#"DROP VIEW IF EXISTS finance.expense_with_receipt "#;
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
