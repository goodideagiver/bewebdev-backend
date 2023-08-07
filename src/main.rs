use std::{error::Error, net::SocketAddr};

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let app = Router::new().route("/", get(|| async { "Hello, Axum" }));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
