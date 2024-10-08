use argon2::Argon2;
use argon2::password_hash::Decimal;
use axum::extract::DefaultBodyLimit;
use axum::{Extension, Json, Router};
use clap::Parser;
use sea_orm::{ConnectionTrait, ConnectOptions, Database, QueryResult, QuerySelect, Statement};

use crate::utils::Error;
use migration::{Migrator, MigratorTrait};
use tracing::log::LevelFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod authorization;
mod cli;
mod schema;
mod specific;
mod utils;

use entities::expense::{CreateExpense, Entity as ExpenseEntity, Model as Expense};
use sea_orm::EntityTrait;
use tracing::info;

const MAX_DB_CONNECTIONS: u32 = 20;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = cli::Args::parse();
    tracing::debug!(
        "service started: {} {}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let app = Router::<_>::new()
        .nest("/api/data", specific::router())
        .merge(
            SwaggerUi::new("/api/data/swagger-ui")
                .url("/api/data/api-doc/openapi.json", schema::Docs::openapi()),
        )
        .nest("/api/auth", auth::router());
    let max_connections = args.max_db_connections.unwrap_or(MAX_DB_CONNECTIONS);
    let mut opts = ConnectOptions::new(args.postgres_url);
    opts.max_connections(max_connections)
        .sqlx_logging_level(LevelFilter::Trace);
    let pool = Database::connect(opts)
        .await
        .expect("cannot connect to postgres");
    if let Err(err) = Migrator::up(&pool, None).await {
        tracing::error!("cannot apply migrations: {:?}", err);
        tracing::error!("exit");

        return;
    }
    let app = app
        .layer(Extension(pool.clone()))
        .layer(Extension(Argon2::default()))
        .layer(DefaultBodyLimit::disable());
    let listener = tokio::net::TcpListener::bind(args.listen_addr)
        .await
        .unwrap();
    info!("service started: {}", args.listen_addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
