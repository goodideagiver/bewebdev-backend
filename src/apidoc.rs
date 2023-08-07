use crate::hello_axum::__path_hello_axum;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(hello_axum))]
pub struct ApiDoc;
