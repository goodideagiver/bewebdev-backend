use axum::{http::StatusCode, response::IntoResponse};

#[utoipa::path(
    get,
    path = "/comments",
    responses(
        (status = 200, description = "List all comments")
    )
)]
pub async fn comments() -> impl IntoResponse {
    (StatusCode::OK, "This is comments")
}
