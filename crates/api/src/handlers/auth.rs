use axum::{http::StatusCode, Json};
use shared::dto::auth::{RegisterRequest, RegisterResponse};
use uuid::Uuid;

pub async fn register(
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<RegisterResponse>) {
    let response = RegisterResponse {
        id: Uuid::new_v4(),
        email: payload.email,
        message: "User registered successfully".to_string(),
    };

    (StatusCode::CREATED, Json(response))
}