use std::{env, net::Ipv4Addr};

use axum::{
    routing::{get, post},
    Router,
};
use deadpool::Runtime;
use deadpool_postgres::{tokio_postgres::NoTls, Config, ManagerConfig};
use tokio::net::TcpListener;

pub mod db;
pub mod error;
pub mod routes;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    // console_subscriber::init();


    let host = Ipv4Addr::new(0, 0, 0, 0);
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let db_host = env::var("DB_HOST").unwrap_or("localhost".to_owned());
    let db_port = env::var("DB_PORT").unwrap_or("5432".to_owned());
    let db_name = env::var("DB_NAME").unwrap_or("rinha".to_owned());

    let mut pool_config = Config::new();
    pool_config.host = Some(db_host);
    pool_config.port = Some(db_port.parse().unwrap());
    pool_config.dbname = Some(db_name);
    pool_config.user = Some("postgres".to_owned());
    pool_config.password = Some("123".to_owned());

    pool_config.manager = Some(ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    let pool = pool_config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .unwrap();

    let router = Router::new()
        .route("/clientes/:id/extrato", get(routes::get_client_statement))
        .route("/clientes/:id/transacoes", post(routes::create_transaction))
        .with_state(pool);

    let tcp_listener = TcpListener::bind((host, port.parse().unwrap()))
        .await
        .unwrap();

    let _ = axum::serve(tcp_listener, router).await.unwrap();
}
