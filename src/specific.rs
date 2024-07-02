use crate::specific::expense::{create_expense, delete_expense, edit_expense, get_expense, get_expenses};
use axum::routing::{delete, get, post, put};
use axum::Router;
pub mod expense;

pub fn router() -> Router {

    let money = Router::new()
        .route("/create", post(create_expense))
        .route("/get_all", post(get_expenses))
        .route("/get/:id", post(get_expense))
        .route("/edit/:id", post(edit_expense))
        .route("/delete/:id", post(delete_expense));


    let finance_router = money;

    Router::new()
        .nest("/expenses", finance_router)
}
