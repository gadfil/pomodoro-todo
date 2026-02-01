use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;
use utoipa::ToSchema;
#[derive(Serialize, ToSchema)]
pub struct ApiError {
    message: String,
    code: u16,
}
impl ApiError {
    pub fn new(code: impl Into<u16>, message: impl Into<String>) -> ApiError {
        Self {
            message: message.into(),
            code: code.into(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
