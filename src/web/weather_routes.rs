use crate::zone_weather_timer::{
    find_cassie_weather, find_crab_weather, find_skoll_weather, handle_bozja_arisu,
};
use axum::response::Html;
use chrono::Utc;
use utils::{format_unix_to_date, format_unix_to_relative_timestr};
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
            <script src="https://lds-img.finalfantasyxiv.com/pc/global/js/eorzeadb/loader.js?v3"></script>
            <p><a href="/eureka" hx-get="/eureka" hx-target="#content">Eureka Weather</a></p>
            <div id="content">
            </div>
        </body>
        </html>"##,
    )
}

fn format_single_weather_condition_card(
    title: &str,
    card_text: &str,
    zone_name: &str,
    is_active: bool,
    weather_time: i64,
) -> String {
    let time_info = format_unix_to_relative_timestr(weather_time)
        + " ("
        + &format_unix_to_date(weather_time)
        + ")";

    format!(
        r#"<div class="weather-card border rounded-xl p-6 bg-gradient-to-br shadow-lg">
        <div class="card-header flex items-start justify-between mb-4 flex-col text-left">
            <h3 class="text-xl font-semibold">{}</h3>
            <h2 class="text-lg font-semibold text-blue-800">{}</h2>
            {}
            </div>

            <div class="card-body">
                <p class="text-gray-600">{}</p>
                <p class="text-gray-800 font-bold mt-2">{}</p>
            </div>
        </div>"#,
        title,
        zone_name,
        if is_active {
            r#"<span class="status-tag text-m align-left px-3 rounded-full bg-green-200">Active</span>"#
        } else {
            r#"<span class="status-tag text-m align-left px-3 rounded-full bg-red-200">Inactive</span>"#
        },
        card_text,
        time_info
    )
}

pub async fn get_eureka_weather_data() -> Html<String> {
    let current_time = Utc::now().timestamp();

    let cassie_weather = find_cassie_weather(current_time);
    let skoll_weather = find_skoll_weather(current_time);
    let crab_weather = find_crab_weather(current_time);

    let cassie_card = format_single_weather_condition_card(
        "Cassie Weather (Blizzards)",
        "Drops <a href=\"https://na.finalfantasyxiv.com/lodestone/playguide/db/item/88febc019e0/\" class=\"eorzeadb_link\">Cassie Earring</a>",
        "Eureka Pagos",
        cassie_weather.0,
        cassie_weather.1,
    );
    let skoll_card = format_single_weather_condition_card(
        "Skoll Weather (Rain)",
        "Drops Skoll Claws",
        "Eureka Pyros",
        skoll_weather.0,
        skoll_weather.1,
    );
    let crab_card = format_single_weather_condition_card(
        "King Arthro [Crab] Weather (Fog)",
        "Drops Blitzring",
        "Eureka Pagos",
        crab_weather.0,
        crab_weather.1,
    );

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
