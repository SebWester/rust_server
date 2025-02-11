// Axum --> HTTP traffic
// Tokio --> Async runtime 
// Tower-http --> Serve static files
use axum::{
    routing::get,
    Router,
    response::Html,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

// Function for homepage
async fn home() -> Html<&'static str> {
    Html("<h1>Welcome to my Axum/Rust server</h1>")
}

// Function for about
async fn about() -> Html<&'static str> {
    Html("<h1>About:</h1> <p>This is a Rust server built with Axum</p>")
}


// Creating a simple server
#[tokio::main]
async fn main() {
    // Create a router and serve files from static folder
    let app: Router = Router::new()        
        .route("/", get(home))
        .route("/about", get(about))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(get(|| async { "404: Page not found"}));

    // Choose port
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀Server running on http://{}", addr);

    // Start server using axum::serve
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    )
    .await
    .unwrap();
}
