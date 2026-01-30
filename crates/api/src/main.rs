use axum::{Router, routing::post, routing::get};
use tokio::net::TcpListener;
use tracing_subscriber;
use shared::config::Config;
mod handlers;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::load().expect("Can't load config");

    let app = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/", get(|| async { "Hello, Pomodoro!" }));
    // let addr = "127.0.0.1:3000";
    let addr = format!("{}:{}", &config.server.host, &config.server.port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    
    tracing::info!("Server running on {}", &addr.clone());
    
    axum::serve(listener, app).await.unwrap();
}
