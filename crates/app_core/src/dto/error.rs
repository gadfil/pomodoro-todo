use serde::Serialize;
use utoipa::ToSchema;
use axum::response::Response;
use axum::Json;
use axum::response::IntoResponse;
#[derive(Serialize, ToSchema)]
pub struct ApiError {
    message: String,
    code: u16
}
impl ApiError {
    pub fn new(code: u16, message:  impl Into<String>) -> ApiError {
        Self { message: message.into(), code }
    }
}
impl IntoResponse for ApiError{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }

}