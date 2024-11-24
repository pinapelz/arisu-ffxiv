use crate::zone_weather_timer::{handle_bozja_arisu, handle_eureka_arisu};
use axum::response::Html;
use chrono::Utc;

pub async fn homepage() -> Html<&'static str> {
    Html(
        r##"<!DOCTYPE html>
        <html>
        <head>
            <title>Arisu Tracker</title>
            <link rel="stylesheet" href="/static/styles.css">
            <script src="https://unpkg.com/htmx.org"></script>
        </head>
        <body>
            <h1>Welcome to Arisu Tracker</h1>
            <p><a href="/eureka" hx-get="/eureka" hx-target="#content">Eureka Weather</a></p>
            <p><a href="/bozja" hx-get="/bozja" hx-target="#content">Bozja Weather</a></p>
            <div id="content">
                <p>Click on a link to load content dynamically.</p>
            </div>
        </body>
        </html>"##,
    )
}

pub async fn eureka_handler() -> Html<String> {
    let current_time_seconds = Utc::now().timestamp();
    Html(format!(
        r#"<p>Eureka Weather at {}</p>"#,
        handle_eureka_arisu(current_time_seconds)
    ))
}

pub async fn bozja_handler() -> Html<String> {
    let current_time_seconds = Utc::now().timestamp();
    Html(format!(
        r#"<p>Bozja Weather at {}</p>"#,
        handle_bozja_arisu(current_time_seconds)
    ))
}
