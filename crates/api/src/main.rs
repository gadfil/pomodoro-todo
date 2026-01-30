use axum::{Router, routing::post, routing::get};
use tokio::net::TcpListener;
use tracing_subscriber;

mod handlers;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/", get(|| async { "Hello, Pomodoro!" }));
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    
    tracing::info!("Server running on http://{}", addr);
    
    axum::serve(listener, app).await.unwrap();
}
