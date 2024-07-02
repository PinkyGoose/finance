use sea_orm::{ DatabaseConnection, EntityTrait, IntoActiveModel};
use serde::Deserialize;
use utoipa::ToSchema;
use entities::{expense::{CreateExpense, Entity as ExpenseEntity}};
use axum::{Extension, Json};
use crate::utils::{CreatedEntity, Error};

#[derive(Clone, Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseQuery{

}


#[utoipa::path(post, path = "/expenses/expense",
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
    let arm = ExpenseEntity::insert(payload.into_active_model())
        .exec_with_returning(pool)
        .await
        .map_err(Error::DatabaseInternal)
        .map(|arm| Json(CreatedEntity::new(arm.id)))?;
    Ok(arm)
}
