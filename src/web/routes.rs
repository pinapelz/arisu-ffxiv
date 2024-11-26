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
            <script src="https://unpkg.com/htmx.org"></script>
            <script src="https://cdn.tailwindcss.com"></script>

        </head>
        <body>
            <p><a href="/eureka" hx-get="/eureka" hx-target="#content">Eureka Weather</a></p>
            <div id="content">
            </div>
        </body>
        </html>"##,
    )
}

fn format_weather_card(title: &str, is_active: bool, weather_time: i64) -> String {
    let status = if is_active {
        format!("Currently Active")
    } else {
        format!("Next Occurrence")
    };

    let time_info = format_unix_to_date(weather_time);

    format!(
        r#"<div class="weather-card border rounded-xl p-6 bg-gradient-to-br from-blue-50 to-blue-100 shadow-lg">
            <div class="card-header flex items-center justify-between mb-4">
                <h3 class="text-lg font-semibold text-blue-800">{}</h3>
                <span class="status-tag text-sm px-3 py-1 rounded-full bg-blue-200 text-blue-800">{}</span>
            </div>
            <div class="card-body">
                <p class="text-gray-600">{}</p>
                <p class="text-gray-800 font-bold mt-2">{}</p>
            </div>
        </div>"#,
        title,
        status,
        if is_active {
            "This weather is currently active and ongoing."
        } else {
            "This weather will occur next at the following time."
        },
        time_info
    )
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
