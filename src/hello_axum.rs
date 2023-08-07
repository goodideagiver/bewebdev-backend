use axum::{http::StatusCode, response::IntoResponse};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Send a salute from Axum")
    )
)]
pub async fn hello_axum() -> impl IntoResponse {
    (StatusCode::OK, "Hello, Axum!")
}
