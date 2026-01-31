use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use app_core::config::Config;
use app_core::dto::auth::{RegisterRequest, RegisterResponse};
use app_core::state::AppState;

mod handlers;
mod routers;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::auth::register),
    components(schemas(RegisterRequest, RegisterResponse)),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::load().expect("Can't load config");

    let addr = format!("{}:{}", &config.server.host, &config.server.port);
    let state = AppState::new(config)
        .await
        .expect("Failed to create app state");

    let app = Router::new()
        .nest("/auth", routers::auth::router())
        .route("/", get(|| async { "Hello, Pomodoro 🍅 " }))
        .with_state(state)
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()));
    let listener = TcpListener::bind(&addr).await.unwrap();

    let url = format!("http://{}", addr);
    let clickable = format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, url);

    println!();
    println!("🍅 Pomodoro Todo API");
    println!();
    println!("Server running at: {}", clickable);
    println!("API docs at:       {}/docs", url);
    println!();

    axum::serve(listener, app).await.unwrap();
}
