use crate::specific::expense::{
    create_expense, delete_expense, edit_expense, get_expense, get_expenses,
};
use axum::routing::{delete, get, post, put};
use axum::Router;
use sea_orm::{DeriveEntityModel, DeriveIntoActiveModel};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

pub mod expense;
mod receipt;
mod expense_with_receipt;

#[derive(Clone, Debug, PartialEq, Eq,Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserId{
    id: Uuid
}


pub fn router() -> Router {
    let money = Router::new()
        .route("/create", post(create_expense))
        .route("/get_all", post(get_expenses))
        .route("/get/:id", post(get_expense))
        .route("/edit/:id", post(edit_expense))
        .route("/delete/:id", post(delete_expense));

    let finance_router = money;

    Router::new().nest("/expenses", finance_router)
}
