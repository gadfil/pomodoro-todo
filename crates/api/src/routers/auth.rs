use crate::handlers::auth::{register, verified_email};
use app_core::state::AppState;
use axum::Router;
use axum::routing::post;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/verified_email", post(verified_email))
}
