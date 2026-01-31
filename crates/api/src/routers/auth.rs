use axum::Router;
use axum::routing::post;
use app_core::state::AppState;
use crate::handlers::auth::register;

pub fn router() ->Router<AppState>{
    Router::new()
        .route("/register", post(register))

}