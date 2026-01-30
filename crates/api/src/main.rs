use axum::{Router, routing::post, routing::get};
use tokio::net::TcpListener;
use tracing_subscriber;
use shared::config::Config;
use domain::state::AppState;

mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::load().expect("Can't load config");


    let addr = format!("{}:{}", &config.server.host, &config.server.port);
    let state = AppState::new(config)
        .await
    .expect("Failed to create app state");

    let app = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/", get(|| async { "Hello, Pomodoro!" }))
        .with_state(state);
    let listener = TcpListener::bind(&addr).await.unwrap();

    let url = format!("http://{}", addr);
    let clickable = format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, url);

    println!();
    println!("🍅 Pomodoro Todo API");
    println!();
    println!("Server running at: {}", clickable);
    println!();
    
    axum::serve(listener, app).await.unwrap();
}
