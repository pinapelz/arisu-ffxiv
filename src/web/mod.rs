use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use chrono::Utc;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub mod weather_routes;

pub async fn start_web_server() {
    let app = Router::new()
        .route("/", get(weather_routes::homepage))
        .route("/eureka", get(weather_routes::get_eureka_weather_data))
        .route("/bozja", get(weather_routes::bozja_handler))
        .nest_service("/static", ServeDir::new("src/web/static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Serving on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
