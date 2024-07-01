use crate::specific::expense::{create_expense, root};
use axum::routing::{get, post};
use axum::Router;
pub mod expense;

pub fn router() -> Router {

    let money = Router::new()
        .route("/expenses", post(create_expense))
        // .route("/expense", post(expense::create_expense))//.get(expence::get_expenses))
        // .route(
        //     "/expense/:id",
        //     get(expence::get_expense).put(expence::edit_expense).delete(expence::delete_expense),
        // )
        ;


    let finance_router = money;

    Router::new()
        .nest("/expenses", finance_router)
}
