use crate::{
    error::Error,
    utils::{AffectedRows, CreatedEntity, Json},
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Extension,
};
use entities::{
    expence::{CreateExpence, Entity as moneyEntity, Model as money, UpdateExpence},
};
use sea_orm::{entity::*, query::*, sea_query, DatabaseConnection};
use sea_query::extension::postgres::PgExpr;
use serde::Deserialize;
use tracing::instrument;
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct moneyQuery {
    offset: Option<u64>,
    limit: Option<u64>,
    hash: Option<String>,
    r#type: Option<String>,
    name: Option<String>,
    #[serde(default = "default_archived")]
    archived: Option<bool>,
}
pub fn default_archived() -> Option<bool> {
    Some(false)
}

#[utoipa::path(post, path = "/security/money",
request_body = Createmoney,
responses(
(status = 200, description = "Успешное создание АРМа", body = money),
(status = 500, description = "Ошибка исполнения запроса")
)
)]

/// Создание АРМа
pub(super) async fn create_money(
    Extension(ref subscription): Extension<Subscriptions>,
    Extension(ref pool): Extension<DatabaseConnection>,
    Json(payload): Json<Createmoney>,
) -> Result<Json<CreatedEntity>, Error> {
    let money = moneyEntity::insert(payload.into_active_model())
        .exec_with_returning(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(|money| Json(CreatedEntity::new(money.id)))?;

    subscription.emit::<entities::money_list::Entity, _>(SubscriptionAction::Create, money.0.id);
    Ok(money)
}

/// Получение списка АРМов с фильтрацией
#[instrument(skip_all, err)]
#[utoipa::path(get, path = "/security/money",
    responses(
        (status = 200, description = "Успешное получение АРМов", body = [money]),
        (status = 500, description = "Ошибка исполнения запроса")
    ),
    params(
        ("filter" = moneyQuery, Query, description = "Параметры поиска АРМов")
    ),
)]
pub(super) async fn get_moneys(
    Extension(ref pool): Extension<DatabaseConnection>,
    Query(q): Query<moneyQuery>,
) -> Result<Json<Vec<money>>, Error> {
    use entities::money::Column;
    let mut query = moneyEntity::find();
    if let Some(offset) = q.offset {
        query = query.offset(offset);
    }
    if let Some(limit) = q.limit {
        query = query.limit(limit);
    }

    if let Some(name) = q.name {
        query = query.filter(sea_query::Expr::col(Column::Name).ilike(format!("{}%", name)))
    }
    if let Some(hash) = q.hash {
        query = query.filter(Column::Hash.eq(hash));
    }

    if let Some(type_money) = q.r#type {
        query = query.filter(Column::Type.eq(type_money));
    }
    if let Some(archived) = q.archived {
        query = query.filter(Column::Archived.eq(archived));
    }
    let moneys = query.all(pool).await.map_err(Error::DatabaseInternal)?;
    Ok(Json(moneys))
}

/// Получение АРМа по его ID
#[instrument(skip_all, err, fields(money_id = id.to_string()))]
#[utoipa::path(get, path = "/security/money/{id}",
    responses(
        (status = 200, description = "Успешное получение АРМа по его ID", body = money),
        (status = 404, description = "Не найден АРМ с заданным ID"),
        (status = 500, description = "Ошибка исполнения запроса")
    ),
    params(
        ("id" = Uuid, Path, description = "ID запрашиваемого АРМа")
    ),
)]
pub(super) async fn get_money(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<money>, Error> {
    moneyEntity::find_by_id(id)
        .one(pool)
        .await
        .map_err(Error::DatabaseInternal)?
        .map(Json)
        .ok_or(Error::NotFound)
}

/// Получение элементов в представлении списка АРМОВ
///
///
#[utoipa::path(get, path = "/security/money_list",
    responses(
        (status = 200, description = "Успешное получение списка АРМов", body = moneyList),
        (status = 404, description = "Отсутствующая сущность")
    ),
    params(
        ("filter" = moneyQuery, Query, description = "Параметры поиска АРМов")
    ),
)]
#[instrument(skip_all, err)]
pub(super) async fn get_money_list(
    Extension(ref pool): Extension<DatabaseConnection>,
    Query(q): Query<moneyQuery>,
) -> Result<impl IntoResponse, Error> {
    let mut query = moneyListEntity::find();
    if let Some(offset) = q.offset {
        query = query.offset(offset);
    }
    if let Some(limit) = q.limit {
        query = query.limit(limit);
    }

    if let Some(name) = q.name {
        query = query.filter(sea_query::Expr::col(Column::Name).ilike(format!("{}%", name)))
    }
    if let Some(hash) = q.hash {
        query = query.filter(Column::Hash.eq(hash));
    }

    if let Some(type_money) = q.r#type {
        query = query.filter(Column::Type.eq(type_money));
    }
    if let Some(archived) = q.archived {
        query = query.filter(Column::Archived.eq(archived));
    }
    let moneys = query.all(pool).await.map_err(Error::DatabaseInternal)?;
    Ok(Json(moneys))
}

/// Изменение существующего АРМа
#[instrument(skip_all, err, fields(money_id = id.to_string()))]
#[utoipa::path(put, path = "/security/money/{id}",
request_body = Updatemoney,
responses(
(status = 200, description = "Данные АРМа успешно изменены", body = money),
(status = 404, description = "Не найден АРМ с заданным ID"),
(status = 500, description = "Ошибка исполнения запроса")
),
params(
("id" = Uuid, Path, description = "ID запрашиваемого АРМа")
),
)]
pub(super) async fn edit_money(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Extension(ref subscription): Extension<Subscriptions>,
    Json(payload): Json<Updatemoney>,
) -> Result<Json<money>, Error> {
    let model = moneyEntity::find_by_id(id)
        .one(pool)
        .await
        .map_err(Error::DatabaseInternal)?
        .ok_or(Error::NotFound)?;

    let model = model.into_active_model();

    let mut new_model = payload.into_active_model();
    new_model.id = model.id;

    let money = new_model
        .update(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(Json)?;

    subscription.emit::<entities::money_list::Entity, _>(SubscriptionAction::Update, money.0.id);
    Ok(money)
}

/// Удаление АРМа по ID
#[instrument(skip_all, err, fields(money_id = id.to_string()))]
#[utoipa::path(delete, path = "/security/money/{id}",
    responses(
        (status = 200, description = "Получен объект АРМа по его ID", body = AffectedRows),
        (status = 500, description = "Ошибка исполнения запроса")
    ),
    params(
        ("id" = Uuid, Path, description = "ID запрашиваемого АРМа")
    ),
)]
pub(super) async fn delete_money(
    Extension(ref pool): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Extension(ref subscription): Extension<Subscriptions>,
) -> Result<Json<AffectedRows>, Error> {
    if let Ok(Some(money)) = moneyEntity::find_by_id(id)
        .one(pool)
        .await
        .map_err(|err| println!("{:?}", err))
    {
        if let Some(layout_id) = money.layout_id {
            let _ = delete_layout(Extension(pool.clone()), axum::extract::Path(layout_id)).await;
        }
    }
    let result = moneyEntity::delete_by_id(id).exec(pool).await?;
    subscription.emit::<entities::money_list::Entity, _>(SubscriptionAction::Delete, id);

    Ok(Json(AffectedRows::new(result.rows_affected)))
}

/// Подписка на Server-Sent Events обновлений сущности moneyList
///
///
#[utoipa::path(post, path = "/updates/money",
    responses(
        (status = 200, description = "Успешная подписка на сущность", body = moneyListSub),
        (status = 404, description = "Отсутствующая сущность")
    ),
)]
#[instrument(skip_all, err)]
pub(super) async fn get_money_updates(
    Extension(ref subscription): Extension<Subscriptions>,
) -> Result<impl IntoResponse, Error> {
    subscription
        .subscribe_sse::<entities::money_list::Entity>()
        .ok_or(Error::NotFound)
}
