use axum::{Extension, Json};
use axum::extract::Query;
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryOrder, QueryResult, Statement};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::specific::expense::{DatePeriod, ExpenseResp, Field, Pagination, Sorting};
use crate::specific::UserId;
use crate::utils::Error;
use entities::exp_rec_list::{Entity as ExpRecpEntity, Model as ExpRecp};
use sea_orm::QueryFilter;
#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseReceiptResp{
    expenses_receipts: Vec<ExpRecp>,
    expense_amount: f64,
    count: i64,
    total_sallary: f64,
    balance: f64
}

#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpReceiptQuery {
    sort: Option<Sorting>,
    pagination: Option<Pagination>,
    period: Option<DatePeriod>,
}



/// Получение затрат с фильтрацией
#[utoipa::path(post, path = "/expenses/get_all",
request_body = ExpenseQuery,
responses(
(status = 200, description = "Успешное получение Затрат", body = [Expense]),
(status = 500, description = "Ошибка исполнения запроса")
)
)]
pub(crate) async fn get_expenses(
    Extension(ref pool): Extension<DatabaseConnection>,
    Query(user_id): Query<UserId>,
    Json(q): Json<ExpReceiptQuery>,
) -> Result<Json<ExpenseReceiptResp>, Error> {
    tracing::info!("get expenses");
    use entities::expense::Column;
    let mut query = ExpRecpEntity::find();
    // if let Some(pagination) = q.pagination {
    //     if let Some(offset) = pagination.offset {
    //         query = query.offset(offset);
    //     }
    //     if let Some(limit) = pagination.limit {
    //         query = query.limit(limit);
    //     }
    // }
    query = query.filter(Column::UserId.eq(user_id.id));
    if let Some(datas) = q.period {
        if let Some(start) = datas.start {
            query = query.filter(Column::CreatedAt.gte(start));
        }
        if let Some(stop) = datas.stop {
            query = query.filter(Column::CreatedAt.lte(stop));
        }
    }

    let expenses = query.all(pool).await.map_err(Error::DatabaseInternal)?;
    let sql = r#"
        select sum(expense_amount)::float as total_expense, sum(total_daily_sallary)::float as total_sallary, count(total_daily_sallary) as total_count
            from finance.expense_with_receipt where user_id =
        "#;
    let query_res: Option<QueryResult> = pool
        .query_one(Statement::from_string(
            pool.get_database_backend(),
            format!("{}{}",sql, user_id.id),
        ))
        .await.unwrap();
    let query_res = query_res.unwrap();
    let total_expense: f64 = query_res.try_get("", "total_expense").unwrap();
    let total_sallary: f64 = query_res.try_get("", "total_sallary").unwrap();
    let total_count: i64 = query_res.try_get("", "total_count").unwrap();
    let balance = total_sallary-total_expense;
    let resp = Json(ExpenseReceiptResp{
        expenses_receipts: expenses,
        expense_amount: total_expense,
        count: total_count,
        total_sallary: total_sallary,
        balance,
    });
    Ok(resp)
}