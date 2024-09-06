use axum::extract::{Path, Query};
use axum::{Extension, Json};
use entities::receipt::{Column, UpdateReceipt};
use entities::receipt::{CreateReceipt, Entity as ReceiptEntity, Model as Receipt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryResult, QuerySelect};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::{CreatedEntity, Error};
use sea_orm::{entity::*, query::*, sea_query};
use sea_query::extension::postgres::PgExpr;
use tracing::instrument;
use uuid::Uuid;
use crate::specific::expense::DatePeriod;
use crate::specific::UserId;

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptsResp{
    receipts: Vec<entities::receipt::Model>,
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
pub struct ReceiptQuery {
    period: Option<DatePeriod>,
    name: Option<String>,
    
}


///Создание затрат
#[utoipa::path(post, path = "/receipts/create",
request_body = CreateReceipt,
responses(
(status = 200, description = "Успешное создание Затрат", body = Receipt),
(status = 500, description = "Ошибка исполнения запроса")
)
)]
pub(crate) async fn create_receipt(
    Extension(ref pool): Extension<DatabaseConnection>,
    Query(user_id): Query<UserId>,
    Json(payload): Json<CreateReceipt>,
) -> Result<Json<CreatedEntity>, Error> {
    tracing::info!("create receipt");
    let mut exp = payload.into_active_model();
    exp.set(Column::UserId, user_id.id.into());
    let arm = ReceiptEntity::insert(exp)
        .exec_with_returning(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(|arm| Json(CreatedEntity::new(arm.id)))?;
    Ok(arm)
}
/// Получение затрат с фильтрацией
#[utoipa::path(post, path = "/receipts/get_all",
request_body = ReceiptQuery,
responses(
(status = 200, description = "Успешное получение Затрат", body = [Receipt]),
(status = 500, description = "Ошибка исполнения запроса")
)
)]
pub(crate) async fn get_receipts(
    Extension(ref pool): Extension<DatabaseConnection>,
    Query(user_id): Query<UserId>,
    Json(q): Json<ReceiptQuery>,
) -> Result<Json<ReceiptsResp>, Error> {
    tracing::info!("get receipts");
    use entities::receipt::Column;
    let mut query = ReceiptEntity::find();
    query = query.filter(entities::expense::Column::UserId.eq(user_id.id));
    if let Some(datas) = q.period {
        if let Some(start) = datas.start {
            query = query.filter(Column::StartDate.gte(start));
        }
        if let Some(stop) = datas.stop {
            query = query.filter(Column::StartDate.lte(stop));
        }
    }
    if let Some(name)= q.name{
        query = query.filter(Column::Name.like(name));
    }

    let receipts = query.all(pool).await.map_err(Error::DatabaseInternal)?;

    let resp = Json(ReceiptsResp{
        receipts
    });
    Ok(resp)
}

/// Изменение затрат по id
#[instrument(skip_all, err, fields(receipt_id = id.to_string()))]
#[utoipa::path(post, path = "/receipts/edit/{id}",
request_body = UpdateReceipt,
responses(
(status = 200, description = "Данные затрат успешно изменены", body = Receipt),
(status = 404, description = "Не найдены затраты с заданным ID"),
(status = 500, description = "Ошибка исполнения запроса")
),
params(
("id" = Uuid, Path, description = "ID редактируемых затрат")
),
)]
pub(super) async fn edit_receipt(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Query(user_id): Query<UserId>,
    Json(payload): Json<UpdateReceipt>,
) -> Result<Json<Receipt>, Error> {
    tracing::info!("edit receipt");
    let model = ReceiptEntity::find_by_id(id).filter(entities::expense::Column::UserId.eq(user_id.id))
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
#[instrument(skip_all, err, fields(receipt_id = id.to_string()))]
#[utoipa::path(post, path = "/receipts/get/{id}",
    responses(
        (status = 200, description = "Успешное получение затрат по ID", body = Receipt),
        (status = 404, description = "Не найдены затраты с заданным ID"),
        (status = 500, description = "Ошибка исполнения запроса")
    ),
    params(
        ("id" = Uuid, Path, description = "ID запрашиваемых затрат")
    ),
)]
pub(super) async fn get_receipt(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Query(user_id): Query<UserId>,
) -> Result<Json<Receipt>, Error> {
    tracing::info!("get receipt");
    ReceiptEntity::find_by_id(id).filter(entities::expense::Column::UserId.eq(user_id.id))
        .one(pool)
        .await
        .map_err(Error::DatabaseInternal)?
        .map(Json)
        .ok_or(Error::NotFound)
}

/// Удаление затрат по ID
#[instrument(skip_all, err, fields(receipt_id = id.to_string()))]
#[utoipa::path(post, path = "/receipts/delete/{id}",
    responses(
        (status = 200, description = "Удалены затраты по ID", body = AffectedRows),
        (status = 500, description = "Ошибка исполнения запроса")
    ),
    params(
        ("id" = Uuid, Path, description = "ID удаляемых затрат")
    ),
)]
pub(super) async fn delete_receipt(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Query(user_id): Query<UserId>,
) -> Result<Json<AffectedRows>, Error> {
    tracing::info!("delete receipt");
    let result = ReceiptEntity::delete_by_id(id).filter(entities::expense::Column::UserId.eq(user_id.id)).exec(pool).await?;

    Ok(Json(AffectedRows::new(result.rows_affected)))
}
