use ffxiv_chronowatcher::eorzean_weather::{find_next_weather_occurance, get_weather_by_time, calculate_current_weather_interval, Weather};

mod utils{
    pub mod time_converter;
}
use utils::time_converter::{convert_to_time_relative_string, convert_to_time_string};

pub fn handle_eureka_arisu(current_time: i64) -> String {
    let current_pyros_weather = get_weather_by_time("Eureka Pyros", current_time);
    let time_until_next_cycle = calculate_current_weather_interval(current_time);
    let mut return_str = String::from("-- Notable Eureka Weather --");
    if current_pyros_weather == Weather::Blizzards {
        return_str.push_str(
            &("\n - Its Skoll Weather Right Now (Blizzards)! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    } else {
        let next_skoll_weather = find_next_weather_occurance("Eureka Pyros", current_time, Weather::Blizzards);
        return_str.push_str(
            &("\n - Skoll Weather (Blizzards) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_skoll_weather.start_time) + " (" + &convert_to_time_string(next_skoll_weather.start_time) + ") ")
        );
    }
    let current_pagos_weather = get_weather_by_time("Eureka Pagos", current_time);

    if current_pagos_weather == Weather::Fog {
        return_str.push_str(
            &("\n - Its Crab Weather Right Now (Fog)! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    } else {
        let next_cassie_weather = find_next_weather_occurance("Eureka Pagos", current_time, Weather::Fog);
        return_str.push_str(
            &("\n - Crab Weather (Fog) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_cassie_weather.start_time) + " (" + &convert_to_time_string(next_cassie_weather.start_time) + ") ")
        );
    }
    if current_pagos_weather == Weather::Blizzards {
        return_str.push_str(
            &("\n - Its Cassie Weather Right Now (Blizzards)! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    } else {
        let next_cassie_weather = find_next_weather_occurance("Eureka Pagos", current_time, Weather::Blizzards);
        return_str.push_str(
            &("\n - Cassie Weather (Blizzards) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_cassie_weather.start_time) + " (" + &convert_to_time_string(next_cassie_weather.start_time) + ") ")
        );
    }
    return_str
}

pub fn handle_bozja_arisu(current_time: i64) -> String {
    let mut return_str = String::from("-- Bozja Weather for Sprite Reflecting --");
    let time_until_next_cycle = calculate_current_weather_interval(current_time);
    let bsf_weather = get_weather_by_time("Bozjan Southern Front", current_time);
    if bsf_weather == Weather::DustStorms {
        return_str.push_str(
            &("\n - Dust Storms (ZONE 2) Now! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    }
    else{
        let next_bsf_dust = find_next_weather_occurance("Bozjan Southern Front", current_time, Weather::DustStorms);
        return_str.push_str(
            &("\n - Dust Storms (ZONE 2) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_bsf_dust.start_time) + " (" + &convert_to_time_string(next_bsf_dust.start_time) + ") ")
        );
    }

    if bsf_weather == Weather::Wind {
        return_str.push_str(
            &("\n - Wind (ZONE 3) Now! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    }
    else{
        let next_bsf_wind = find_next_weather_occurance("Bozjan Southern Front", current_time, Weather::Wind);
        return_str.push_str(
            &("\n - Wind (ZONE 3) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_bsf_wind.start_time) + " (" + &convert_to_time_string(next_bsf_wind.start_time) + ") ")
        );
    }

    if bsf_weather == Weather::Thunder {
        return_str.push_str(
            &("\n - Thunder (ZONE 1) Now! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    }
    else{
        let next_bsf_thunder = find_next_weather_occurance("Bozjan Southern Front", current_time, Weather::Thunder);
        return_str.push_str(
            &("\n - Thunder (ZONE 1) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_bsf_thunder.start_time) + " (" + &convert_to_time_string(next_bsf_thunder.start_time) + ") ")
        );
    }

    return_str.push_str("\n\n-- Zadnor --");
    let zadnor_weather = get_weather_by_time("Zadnor", current_time);

    if zadnor_weather == Weather::Snow {
        return_str.push_str(
            &("\n - Snow (ZONE 3) Now! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    }
    else{
        let next_zadnor_snow = find_next_weather_occurance("Zadnor", current_time, Weather::Snow);
        return_str.push_str(
            &("\n - Snow (ZONE 3) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_zadnor_snow.start_time) + " (" + &convert_to_time_string(next_zadnor_snow.start_time) + ") ")
        );
    }

    if zadnor_weather == Weather::Wind {
        return_str.push_str(
            &("\n - Wind (ZONE 1) Now! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    }
    else{
        let next_zadnor_wind = find_next_weather_occurance("Zadnor", current_time, Weather::Wind);
        return_str.push_str(
            &("\n - Wind (ZONE 1) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_zadnor_wind.start_time) + " (" + &convert_to_time_string(next_zadnor_wind.start_time) + ") ")
        );
    }

    if zadnor_weather == Weather::Rain {
        return_str.push_str(
            &("\n - Rain (ZONE 2) Now! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    }
    else{
        let next_zadnor_rain = find_next_weather_occurance("Zadnor", current_time, Weather::Rain);
        return_str.push_str(
            &("\n - Rain (ZONE 2) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_zadnor_rain.start_time) + " (" + &convert_to_time_string(next_zadnor_rain.start_time) + ") ")
        );
    }

    if zadnor_weather == Weather::Thunder {
        return_str.push_str(
            &("\n - Thunder (ZONE 2) Now! Ends in ".to_string() + &convert_to_time_relative_string(current_time, time_until_next_cycle.1))
        );
    }
    else{
        let next_zadnor_thunder = find_next_weather_occurance("Zadnor", current_time, Weather::Thunder);
        return_str.push_str(
            &("\n - Thunder (ZONE 2) In: ".to_string() + &convert_to_time_relative_string(
                current_time, next_zadnor_thunder.start_time) + " (" + &convert_to_time_string(next_zadnor_thunder.start_time) + ") ")
        );
    }

    return_str
}