use crate::authorization::auth::create_user;
use crate::specific::expense::{create_expense, get_expenses};
use axum::routing::post;
use axum::Router;

pub fn router() -> Router {
    let auth = Router::new().route("/create", post(create_user));

    let auth_router = auth;

    Router::new().nest("/expenses", auth_router)
}
