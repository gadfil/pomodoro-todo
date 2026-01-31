use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use app_core::dto::auth::{RegisterRequest, RegisterResponse};
use app_core::errors::AuthError;
use app_core::state::AppState;
use app_core::dto::response::DataResponse;
use app_core::dto::error::ApiError;
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 409, description = "Email already taken", body= ApiError),
        (status = 409, description = "Email address is not confirmed", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
)->Result<(StatusCode, DataResponse<RegisterResponse>), (StatusCode,ApiError)> {
    match app_core::services::auth::register(&state.db, payload).await {
        Ok(response)=>Ok((StatusCode::CREATED,DataResponse::new(response))),
        Err(AuthError::EmailTaken)=>Err((StatusCode::CONFLICT,  ApiError::new(StatusCode::CONFLICT.as_u16(),AuthError::EmailTaken.to_string()))),
        Err(AuthError::EmailNotConfirmed)=>Err((StatusCode::CONFLICT,  ApiError::new(StatusCode::CONFLICT.as_u16(), AuthError::EmailNotConfirmed.to_string()))),
        Err(_)=>Err((StatusCode::INTERNAL_SERVER_ERROR,  ApiError::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "INTERNAL_SERVER_ERROR".to_string())))

    }
}