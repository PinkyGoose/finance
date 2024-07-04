use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QuerySelect};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use entities::{expense::{CreateExpense,Model as Expense, Entity as ExpenseEntity}};
use axum::{Extension, Json};
use axum::extract::{Path, Query};
use entities::expense::{Column, UpdateExpense};

use crate::utils::{CreatedEntity, Error};
use sea_orm::{entity::*, query::*, sea_query};
use tracing::instrument;
use uuid::Uuid;

#[derive(Serialize, ToSchema, Debug)]
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
pub struct ExpenseQuery{
    pagination: Option<Pagination>,
    period: Option<DatePeriod>

}
#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
pub struct Pagination{
    offset: Option<u64>,
    limit: Option<u64>,
}
#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
pub struct DatePeriod{
    start: Option<chrono::DateTime<chrono::FixedOffset>>,
    stop: Option<chrono::DateTime<chrono::FixedOffset>>,
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
    Json(payload): Json<CreateExpense>,
)-> Result<Json<CreatedEntity>, Error>{
    tracing::info!("create expense");
    let arm = ExpenseEntity::insert(payload.into_active_model())
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
    Json(q): Json<ExpenseQuery>,
)-> Result<Json<Vec<Expense>>, Error>{
    tracing::info!("get all expenses");
    use entities::expense::Column;
    let mut query = ExpenseEntity::find();
    if let Some(pagination) = q.pagination {
        if let Some(offset) = pagination.offset {
            query = query.offset(offset);
        }
        if let Some(limit) = pagination.limit {
            query = query.limit(limit);
        }
    }
    if let Some(datas) = q.period {
        if let Some(start) = datas.start {
            query = query.filter(Column::CreatedAt.gte(start));
        }
        if let Some(stop) = datas.stop {
            query = query.filter(Column::CreatedAt.lte(stop));
        }
    }
    let arms = query.all(pool).await.map_err(Error::DatabaseInternal)?;
    Ok(Json(arms))
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
    Json(payload): Json<UpdateExpense>,
) -> Result<Json<Expense>, Error> {
    tracing::info!("edit expense");
    let model = ExpenseEntity::find_by_id(id)
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
) -> Result<Json<Expense>, Error> {
    tracing::info!("get expense");
    ExpenseEntity::find_by_id(id)
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
) -> Result<Json<AffectedRows>, Error> {
    tracing::info!("delete expense");
    let result = ExpenseEntity::delete_by_id(id).exec(pool).await?;
    Ok(Json(AffectedRows::new(result.rows_affected)))
}