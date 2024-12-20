use crate::time_utils::{format_unix_to_date, format_unix_to_relative_timestr};
use crate::zone_weather_timer::{
    find_bsf_dust, find_bsf_thunder, find_bsf_wind, find_cassie_weather, find_crab_weather,
    find_skoll_weather, find_zadnor_rain, find_zadnor_snow, find_zadnor_thunder, find_zadnor_wind,
};
use axum::response::Html;
use chrono::Utc;

fn format_single_weather_condition_card(
    title: &str,
    card_text: &str,
    zone_name: &str,
    is_active: bool,
    weather_time: i64,
) -> String {
    let time_info = if is_active {
        format!(
            "ends {} ({})",
            format_unix_to_relative_timestr(weather_time),
            format_unix_to_date(weather_time)
        )
    } else {
        format!(
            "starts {} ({})",
            format_unix_to_relative_timestr(weather_time),
            format_unix_to_date(weather_time)
        )
    };

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

fn format_multi_weather_condition_card(
    title: &str,
    conditions: Vec<(&str, &str, bool, i64, &str)>,
) -> String {
    let mut rows = String::new();

    for (condition, zone, is_active, time, tooltip_info) in conditions {
        let time_info = if is_active {
            format!(
                "ends {} ({})",
                format_unix_to_relative_timestr(time),
                format_unix_to_date(time)
            )
        } else {
            format!(
                "starts {} ({})",
                format_unix_to_relative_timestr(time),
                format_unix_to_date(time)
            )
        };
        let status = if is_active {
            r#"<span class="status-tag text-xs px-2 py-1 rounded bg-green-200 text-green-800">Active</span>"#
        } else {
            r#"<span class="status-tag text-xs px-2 py-1 rounded bg-red-200 text-red-800">Inactive</span>"#
        };

        rows.push_str(&format!(
            r#"<div class="condition-row flex items-center justify-between text-sm py-2 relative group hover:bg-gray-100 rounded-lg">
                        <div class="condition-info text-left items-center">
                            <span class="font-medium">{}</span>
                        </div>
                        <span class="text-gray-600 text-xl mx-4">{}</span>
                        <div class="flex text-lg items-center">
                            {}
                            <span class="ml-2 text-m font-small text-gray-800">{}</span>
                        </div>
                        <div class="tooltip absolute hidden group-hover:block bg-gray-800 text-white text-xs rounded-lg px-3 py-2 shadow-lg z-10 bottom-full mb-1 w-64">
                            {}
                        </div>
                    </div>
                "#,
            status,condition, zone, time_info, tooltip_info
        ));
    }

    format!(
        r#"<div class="weather-card compact border rounded-lg p-3 bg-white align-left shadow">
            <h3 class="text-base font-semibold mb-2">{}</h3>
            <div class="conditions space-y-1">
                {}
            </div>
        </div>"#,
        title, rows
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
                <div class="weather-cards">
                    {}{}{}
                </div>
            </div>
        </body>
        </html>
        "#,
        cassie_card, skoll_card, crab_card,
    ))
}

pub async fn get_bozja_weather_data() -> Html<String> {
    let current_time = Utc::now().timestamp();
    let bsf_dust = find_bsf_dust(current_time);
    let bsf_wind = find_bsf_wind(current_time);
    let bsf_thunder = find_bsf_thunder(current_time);

    let zadnor_snow = find_zadnor_snow(current_time);
    let zadnor_wind = find_zadnor_wind(current_time);
    let zadnor_thunder = find_zadnor_thunder(current_time);
    let zadnor_rain = find_zadnor_rain(current_time);
    let bsf_minicard = format_multi_weather_condition_card(
        "Bozjan Southern Front Sprite Reflecting",
        vec![
            (
                "Dust",
                "Zone II",
                bsf_dust.0,
                bsf_dust.1,
                "Earth Sprite (V) Drops Lost Fragments of XYZ",
            ),
            (
                "Wind",
                "Zone III",
                bsf_wind.0,
                bsf_wind.1,
                "Wind Sprite (V) Drops Lost Fragments of XYZ",
            ),
            (
                "Lightning",
                "Zone I",
                bsf_thunder.0,
                bsf_thunder.1,
                "Lightning Sprite (V) Drops Lost Fragments of XYZ",
            ),
        ],
    );

    let zadnor_minicard = format_multi_weather_condition_card(
        "Zadnor Sprite Reflecting",
        vec![
            (
                "Snow",
                "Zone II",
                zadnor_snow.0,
                zadnor_snow.1,
                "Snow Sprite (V) Drops Lost Fragments of XYZ",
            ),
            (
                "Wind",
                "Zone III",
                zadnor_wind.0,
                zadnor_wind.1,
                "Wind Sprite (V) Drops Lost Fragments of XYZ",
            ),
            (
                "Lightning",
                "Zone I",
                zadnor_thunder.0,
                zadnor_thunder.1,
                "Lightning Sprite (V) Drops Lost Fragments of XYZ",
            ),
            (
                "Rain",
                "Zone IV",
                zadnor_rain.0,
                zadnor_rain.1,
                "Rain Sprite (V) Drops Lost Fragments of XYZ",
            ),
        ],
    );

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Bozja Weather</title>
        </head>
        <body>
            <div class="bozja-weather">
                <div class="weather-cards">
                    {}
                    {}
                </div>
            </div>
            </div>
        </body>
        </html>
        "#,
        bsf_minicard, zadnor_minicard
    ))
}
