use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum_valid::Valid;
use app_core::dto::auth::{EmailVerifiedRequest, RegisterRequest, RegisterResponse};
use app_core::dto::error::ApiError;
use app_core::dto::response::DataResponse;
use app_core::errors::AuthError;
use app_core::state::AppState;
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
    Valid(Json(payload)): Valid<Json<RegisterRequest>>,
) -> Result<(StatusCode, DataResponse<RegisterResponse>), (StatusCode, ApiError)> {
    match app_core::services::auth::register(&state.db, payload, state.code_sender.as_ref()).await {
        Ok(response) => Ok((StatusCode::CREATED, DataResponse::new(response))),
        Err(AuthError::EmailTaken) => Err((
            StatusCode::CONFLICT,
            ApiError::new(
                StatusCode::CONFLICT.as_u16(),
                AuthError::EmailTaken.to_string(),
            ),
        )),
        Err(AuthError::EmailNotConfirmed) => Err((
            StatusCode::CONFLICT,
            ApiError::new(
                StatusCode::CONFLICT.as_u16(),
                AuthError::EmailNotConfirmed.to_string(),
            ),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "INTERNAL_SERVER_ERROR".to_string(),
            ),
        )),
    }
}

pub async fn verified_email(
    State(state): State<AppState>,
    Valid(Json(payload)): Valid<Json<EmailVerifiedRequest>>,
) -> Result<(StatusCode, DataResponse<()>), (StatusCode, ApiError)> {
    match app_core::services::auth::confirm_email(&state.db, payload.email, payload.code).await {
        Ok(()) => Ok((
            StatusCode::OK,
            DataResponse::new(()),
        )),
        Err(AuthError::TooManyAttempts) => Err((
            StatusCode::TOO_MANY_REQUESTS,
            ApiError::new(
                StatusCode::TOO_MANY_REQUESTS.as_u16(),
                AuthError::TooManyAttempts.to_string(),
            ),
        )),
        Err(AuthError::InvalidCode) => Err((
            StatusCode::BAD_REQUEST,
            ApiError::new(
                StatusCode::BAD_REQUEST.as_u16(),
                AuthError::InvalidCode.to_string(),
            ),
        )),
        Err(AuthError::CodeExpired) => Err((
            StatusCode::GONE,
            ApiError::new(
                StatusCode::GONE.as_u16(),
                AuthError::CodeExpired.to_string(),
            ),
        )),
        Err(AuthError::UserNotFound) => Err((
            StatusCode::NOT_FOUND,
            ApiError::new(
                StatusCode::NOT_FOUND.as_u16(),
                AuthError::UserNotFound.to_string(),
            ),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "INTERNAL_SERVER_ERROR".to_string(),
            ),
        )),
    }
}
