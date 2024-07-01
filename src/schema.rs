
use crate::utils::CreatedEntity;
use crate::specific::expense;
use crate::specific::expense::__path_create_expense;
use crate::specific::expense::ExpenseQuery;
use entities::{
    expense::{CreateExpense, Model as Expense, UpdateExpense},
};
use utoipa::{openapi::Server, Modify, OpenApi};

#[derive(utoipa::ToSchema)]
struct Value {}

#[derive(utoipa::ToSchema)]
struct Json {}
#[derive(OpenApi)]
#[openapi(
    paths(

        expense::create_expense,
    ),
    components(
        schemas(
            CreatedEntity,
            //AffectedRows,
            Value,
            Json,
            // Count,
            // Empty,
        // expense
            Expense, ExpenseQuery, CreateExpense, UpdateExpense,
        )
    ),
    modifiers(&ServerAddon),
)]

pub struct Docs;
struct ServerAddon;

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.servers = Some(vec![Server::new("/api/data")])
    }
}
