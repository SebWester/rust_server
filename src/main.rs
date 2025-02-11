// Axum --> HTTP traffic
// Tokio --> Async runtime 
// Tower-http --> Serve static files
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

// Creating a simple server
#[tokio::main]
async fn main() {
    // Create a router
    let app = Router::new()
        .route("/", get(|| async { "Welcome to my rust server!" }))
        .nest_service("/static", ServeDir::new("static"));        

    // Choose port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀Server running on http://{}", addr);

    // Start server using axum::serve
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    )
    .await
    .unwrap();
}
