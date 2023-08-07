use std::{error::Error, net::SocketAddr};

use axum::{routing::get, Router};

use crate::hello_axum::__path_hello_axum;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(hello_axum))]
pub struct ApiDoc;

mod hello_axum;
use hello_axum::hello_axum;

use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket_address = SocketAddr::from(([127, 0, 0, 1], 6969));

    let app = Router::new()
        .route("/", get(hello_axum))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("listening on {}", socket_address);

    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
