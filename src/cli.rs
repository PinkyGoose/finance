use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    /// Адрес, на котором будет запущен сервис.
    #[clap(long, env, default_value_t = SocketAddr::from(([0, 0, 0, 0], 8080)))]
    pub listen_addr: SocketAddr,

    /// URL соединения с базой данных PostgreSQL.
    #[clap(long, env)]
    pub postgres_url: String,

    /// Максимальное количество подключений к базе данных
    #[clap(long, env)]
    pub max_db_connections: Option<u32>,

}