use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use chrono::Utc;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub mod routes; // Separate file for route handlers

pub async fn start_web_server() {
    // Create a router with defined routes
    let app = Router::new()
        .route("/", get(routes::homepage))
        .route("/eureka", get(routes::eureka_handler))
        .route("/bozja", get(routes::bozja_handler))
        .nest_service("/static", ServeDir::new("web/static")); // Serve static files

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Serving on http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
