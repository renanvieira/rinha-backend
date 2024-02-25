pub mod db;
pub mod routes;
pub mod model;

use std::{env, net::SocketAddr};

use dotenv::dotenv;
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    // let cfg = DBConfig::from_env().unwrap();
    // let mgr_config = ManagerConfig {
    //     recycling_method: RecyclingMethod::Fast,
    // };
    //
    let port = env::var("PORT").unwrap_or("9999".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().expect("POST must be a number")));

    println!("{:?}", env::var("DB_DSN"));
    println!("Listening on http://{}", addr);
    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(routes::get_client_overview))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
