use crate::specific::expense::{
    create_expense, delete_expense, edit_expense, get_expense, get_expenses,
};
use axum::routing::{delete, get, post, put};
use axum::Router;
use sea_orm::{DeriveEntityModel, DeriveIntoActiveModel};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::specific::expense_with_receipt::get_expenses_receipts;
use crate::specific::receipt::{create_receipt, delete_receipt, edit_receipt, get_receipt, get_receipts};

pub mod expense;
mod receipt;
mod expense_with_receipt;

#[derive(Clone, Debug, PartialEq, Eq,Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserId{
    id: Uuid
}


pub fn router() -> Router {
    let expense = Router::new()
        .route("/create", post(create_expense))
        .route("/get_all", post(get_expenses))
        .route("/get/:id", post(get_expense))
        .route("/edit/:id", post(edit_expense))
        .route("/delete/:id", post(delete_expense));

    let expense_router = expense;
    let receipt = Router::new()
        .route("/create", post(create_receipt))
        .route("/get_all", post(get_receipts))
        .route("/get/:id", post(get_receipt))
        .route("/edit/:id", post(edit_receipt))
        .route("/delete/:id", post(delete_receipt));

    let receipt_router = receipt;
    let expense_receipt_list = Router::new()
        .route("/get_all", post(get_expenses_receipts));

    let expense_receipt_list_router = expense_receipt_list;
    Router::new().nest("/expenses", expense_router)
        .nest("/receipts", receipt_router)
        .nest("/expense_receipt_list", expense_receipt_list_router)
}
