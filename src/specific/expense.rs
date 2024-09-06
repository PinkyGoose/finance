use axum::extract::{Path, Query};
use axum::{Extension, Json};
use entities::expense::{Column, UpdateExpense};
use entities::expense::{CreateExpense, Entity as ExpenseEntity, Model as Expense};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryResult, QuerySelect};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::{CreatedEntity, Error};
use sea_orm::{entity::*, query::*, sea_query};
use sea_query::extension::postgres::PgExpr;
use tracing::instrument;
use uuid::Uuid;
use crate::specific::UserId;

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseResp{
    pub(crate) expenses: Vec<entities::expense::Model>,
    sum: f64,
    count: i64
}
#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AffectedRows {
    affected_rows: u64,
}
impl AffectedRows {
    pub fn new(affected_rows: u64) -> Self {
        Self { affected_rows }
    }
}
#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub enum Sort {
    Desc,
    #[default]
    Asc,
}
#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub enum Field {
    Id,
    #[default]
    CreatedAt,
    ValueSum,
}
impl Into<Column> for Field {
    fn into(self) -> Column {
        match self {
            Field::Id => Column::Id,
            Field::CreatedAt => Column::CreatedAt,
            Field::ValueSum => Column::ValueSum,
        }
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sorting {
    field_name: Field,
    order: Sort,
}
impl Into<Order> for Sort {
    fn into(self) -> Order {
        match self {
            Sort::Desc => Order::Desc,
            Sort::Asc => Order::Asc,
        }
    }
}

#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseQuery {
    pub(crate) sort: Option<Sorting>,
    pagination: Option<Pagination>,
    pub(crate) period: Option<DatePeriod>,
}
#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    offset: Option<u64>,
    limit: Option<u64>,
}
#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct DatePeriod {
    pub(crate) start: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub(crate) stop: Option<chrono::DateTime<chrono::FixedOffset>>,
}
///Создание затрат
#[utoipa::path(post, path = "/expenses/create",
request_body = CreateExpense,
responses(
(status = 200, description = "Успешное создание Затрат", body = Expense),
(status = 500, description = "Ошибка исполнения запроса")
)
)]
pub(crate) async fn create_expense(
    Extension(ref pool): Extension<DatabaseConnection>,
    Query(user_id): Query<UserId>,
    Json(payload): Json<CreateExpense>,
) -> Result<Json<CreatedEntity>, Error> {
    tracing::info!("create expense");
    let mut exp = payload.into_active_model();
exp.set(Column::UserId, user_id.id.into());
    let arm = ExpenseEntity::insert(exp)
        .exec_with_returning(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(|arm| Json(CreatedEntity::new(arm.id)))?;
    Ok(arm)
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
    Json(q): Json<ExpenseQuery>,
) -> Result<Json<ExpenseResp>, Error> {
    tracing::info!("get expenses");
    use entities::expense::Column;
    let mut query = ExpenseEntity::find();
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
    if let Some(sort) = q.sort {
        query = query.order_by(
            match sort.field_name {
                Field::Id => Column::Id,
                Field::CreatedAt => Column::CreatedAt,
                Field::ValueSum => Column::ValueSum,
            },
            sort.order.into(),
        );
    }
    let expenses = query.all(pool).await.map_err(Error::DatabaseInternal)?;
    let sql = r#"
        select sum(value_sum)::float as summ, count(value_sum) as countt
            from finance.expense where user_id = 
        "#;
    let query_res: Option<QueryResult> = pool
        .query_one(Statement::from_string(
            pool.get_database_backend(),
            format!("{}{}",sql, user_id.id),
        ))
        .await.unwrap();
    let query_res = query_res.unwrap();
    let sum: f64 = query_res.try_get("", "summ").unwrap();
    let count: i64 = query_res.try_get("", "countt").unwrap();

    let resp = Json(ExpenseResp{
        expenses: expenses,
        sum:sum,
        count:count
    });
    Ok(resp)
}

/// Изменение затрат по id
#[instrument(skip_all, err, fields(expense_id = id.to_string()))]
#[utoipa::path(post, path = "/expenses/edit/{id}",
request_body = UpdateExpense,
responses(
(status = 200, description = "Данные затрат успешно изменены", body = Expense),
(status = 404, description = "Не найдены затраты с заданным ID"),
(status = 500, description = "Ошибка исполнения запроса")
),
params(
("id" = Uuid, Path, description = "ID редактируемых затрат")
),
)]
pub(super) async fn edit_expense(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Query(user_id): Query<UserId>,
    Json(payload): Json<UpdateExpense>,
) -> Result<Json<Expense>, Error> {
    tracing::info!("edit expense");
    let model = ExpenseEntity::find_by_id(id).filter(Column::UserId.eq(user_id.id))
        .one(pool)
        .await
        .map_err(Error::DatabaseInternal)?
        .ok_or(Error::NotFound)?;

    let model = model.into_active_model();

    let mut new_model = payload.into_active_model();
    new_model.id = model.id;

    let arm = new_model
        .update(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(Json)?;

    Ok(arm)
}

/// Получение Затрат по ID
#[instrument(skip_all, err, fields(expense_id = id.to_string()))]
#[utoipa::path(post, path = "/expenses/get/{id}",
    responses(
        (status = 200, description = "Успешное получение затрат по ID", body = Expense),
        (status = 404, description = "Не найдены затраты с заданным ID"),
        (status = 500, description = "Ошибка исполнения запроса")
    ),
    params(
        ("id" = Uuid, Path, description = "ID запрашиваемых затрат")
    ),
)]
pub(super) async fn get_expense(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Query(user_id): Query<UserId>,
) -> Result<Json<Expense>, Error> {
    tracing::info!("get expense");
    ExpenseEntity::find_by_id(id).filter(Column::UserId.eq(user_id.id))
        .one(pool)
        .await
        .map_err(Error::DatabaseInternal)?
        .map(Json)
        .ok_or(Error::NotFound)
}

/// Удаление затрат по ID
#[instrument(skip_all, err, fields(expense_id = id.to_string()))]
#[utoipa::path(post, path = "/expenses/delete/{id}",
    responses(
        (status = 200, description = "Удалены затраты по ID", body = AffectedRows),
        (status = 500, description = "Ошибка исполнения запроса")
    ),
    params(
        ("id" = Uuid, Path, description = "ID удаляемых затрат")
    ),
)]
pub(super) async fn delete_expense(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Query(user_id): Query<UserId>,
) -> Result<Json<AffectedRows>, Error> {
    tracing::info!("delete expense");
    let result = ExpenseEntity::delete_by_id(id).filter(Column::UserId.eq(user_id.id)).exec(pool).await?;

    Ok(Json(AffectedRows::new(result.rows_affected)))
}
