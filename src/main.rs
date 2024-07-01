use axum::{Extension, Router};
use axum::extract::DefaultBodyLimit;
use clap::Parser;

use tracing::log::LevelFilter;
mod cli;
mod specific;

const MAX_DB_CONNECTIONS: u32 = 20;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let args = cli::Args::parse();
    tracing::debug!(
        "service started: {} {}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    // let app = Router::<_>::new()
    //     .nest("/api/data", specific::router())
    //     .route("/metrics", axum::routing::get(metrics::expose_metrics))
    //     .merge(
    //         SwaggerUi::new("/api/data/swagger-ui")
    //             .url("/api/data/api-doc/openapi.json", schema::Docs::openapi()),
    //     );
    // let max_connections = args.max_db_connections.unwrap_or(MAX_DB_CONNECTIONS);
    // let mut opts = ConnectOptions::new(args.postgres_url);
    // opts.max_connections(max_connections)
    //     .sqlx_logging_level(LevelFilter::Trace);
    // let pool = Database::connect(opts)
    //     .await
    //     .expect("cannot connect to postgres");
    // if let Err(err) = Migrator::up(&pool, None).await {
    //     tracing::error!("cannot apply migrations: {:?}", err);
    //     tracing::error!("exit");
    //
    //     return;
    // }
    // let app = app
    //     .layer(Extension(pool.clone()))
    //     .layer(DefaultBodyLimit::disable());
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app.into_make_service).await.unwrap();

}
