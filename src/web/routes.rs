use crate::zone_weather_timer::{
    find_cassie_weather, find_crab_weather, find_skoll_weather, handle_bozja_arisu,
};
use axum::response::Html;
use chrono::Utc;
use utils::format_unix_to_date;
mod utils;

pub async fn homepage() -> Html<&'static str> {
    Html(
        r##"<!DOCTYPE html>
        <html>
        <head>
            <title>Arisu Tracker</title>
            <link rel="stylesheet" href="/static/weather-card.css">

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

fn format_weather_card(title: &str, is_active: bool, weather_time: i64) -> String {
    if is_active {
        format!(
            r#"<div class="weather-card">
                <h3>{}</h3>
                <p>It's currently <strong>{}</strong> NOW!</p>
                <p>Ends at: {}</p>
            </div>"#,
            title,
            title,
            format_unix_to_date(weather_time),
        )
    } else {
        format!(
            r#"<div class="weather-card">
                <h3>{}</h3>
                <p>Next occurrence:</p>
                <p>{}</p>
            </div>"#,
            title,
            format_unix_to_date(weather_time),
        )
    }
}
pub async fn get_eureka_weather_data() -> Html<String> {
    let current_time = Utc::now().timestamp();

    let cassie_weather = find_cassie_weather(current_time);
    let skoll_weather = find_skoll_weather(current_time);
    let crab_weather = find_crab_weather(current_time);

    let cassie_card = format_weather_card(
        "Cassie Weather (Thunder)",
        cassie_weather.0,
        cassie_weather.1,
    );
    let skoll_card = format_weather_card("Skoll Weather (Rain)", skoll_weather.0, skoll_weather.1);
    let crab_card = format_weather_card("Crab Weather (Fog)", crab_weather.0, crab_weather.1);

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Eureka Weather</title>
        </head>
        <body>
            <div class="eureka-weather">
                <p>Eureka Weather at {}</p>
                <div class="weather-cards">
                    {}{}{}
                </div>
            </div>
        </body>
        </html>
        "#,
        format_unix_to_date(current_time),
        cassie_card,
        skoll_card,
        crab_card,
    ))
}

pub async fn bozja_handler() -> Html<String> {
    let current_time_seconds = Utc::now().timestamp();
    Html(format!(
        r#"<p>Bozja Weather at {}</p>"#,
        handle_bozja_arisu(current_time_seconds)
    ))
}
