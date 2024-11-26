use ffxiv_chronowatcher::eorzean_weather::{
    calculate_current_weather_interval, find_next_weather_occurance, get_weather_by_time, Weather,
};

mod utils {
    pub mod time_converter;
}
use utils::time_converter::{convert_to_time_relative_string, convert_to_time_string};

pub fn handle_eureka_arisu(current_time: i64) -> String {
    let mut return_str = String::from("-- Notable Eureka Weather --");

    // Handle Skoll weather in Pyros
    let (is_skoll_active, skoll_time) = find_skoll_weather(current_time);
    if is_skoll_active {
        return_str.push_str(
            &("\n - It's Skoll Weather Right Now (Blizzards)! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, skoll_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Skoll Weather (Blizzards) In: ".to_string()
                + &convert_to_time_relative_string(current_time, skoll_time)
                + " ("
                + &convert_to_time_string(skoll_time)
                + ") "),
        );
    }

    // Handle Crab weather in Pagos
    let (is_crab_active, crab_time) = find_crab_weather(current_time);
    if is_crab_active {
        return_str.push_str(
            &("\n - It's Crab Weather Right Now (Fog)! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, crab_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Crab Weather (Fog) In: ".to_string()
                + &convert_to_time_relative_string(current_time, crab_time)
                + " ("
                + &convert_to_time_string(crab_time)
                + ") "),
        );
    }

    // Handle Cassie weather in Pagos
    let (is_cassie_active, cassie_time) = find_cassie_weather(current_time);
    if is_cassie_active {
        return_str.push_str(
            &("\n - It's Cassie Weather Right Now (Blizzards)! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, cassie_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Cassie Weather (Blizzards) In: ".to_string()
                + &convert_to_time_relative_string(current_time, cassie_time)
                + " ("
                + &convert_to_time_string(cassie_time)
                + ") "),
        );
    }

    return_str
}

pub fn find_skoll_weather(current_time: i64) -> (bool, i64) {
    let current_pyros_weather = get_weather_by_time("Eureka Pyros", current_time);
    if current_pyros_weather == Weather::Blizzards {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_skoll_weather =
            find_next_weather_occurance("Eureka Pyros", current_time, Weather::Blizzards);
        (false, next_skoll_weather.start_time)
    }
}

pub fn find_crab_weather(current_time: i64) -> (bool, i64) {
    let current_pagos_weather = get_weather_by_time("Eureka Pagos", current_time);
    if current_pagos_weather == Weather::Fog {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_crab_weather =
            find_next_weather_occurance("Eureka Pagos", current_time, Weather::Fog);
        (false, next_crab_weather.start_time)
    }
}

pub fn find_cassie_weather(current_time: i64) -> (bool, i64) {
    let current_pagos_weather = get_weather_by_time("Eureka Pagos", current_time);
    if current_pagos_weather == Weather::Blizzards {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_cassie_weather =
            find_next_weather_occurance("Eureka Pagos", current_time, Weather::Blizzards);
        (false, next_cassie_weather.start_time)
    }
}

pub fn handle_bozja_arisu(current_time: i64) -> String {
    let mut return_str = String::from("-- Bozja Weather for Sprite Reflecting --");

    // Bozjan Southern Front Weather
    let (is_dust_active, dust_time) = find_bsf_dust(current_time);
    if is_dust_active {
        return_str.push_str(
            &("\n - Dust Storms (ZONE 2) Now! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, dust_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Dust Storms (ZONE 2) In: ".to_string()
                + &convert_to_time_relative_string(current_time, dust_time)
                + " ("
                + &convert_to_time_string(dust_time)
                + ") "),
        );
    }

    let (is_wind_active, wind_time) = find_bsf_wind(current_time);
    if is_wind_active {
        return_str.push_str(
            &("\n - Wind (ZONE 3) Now! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, wind_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Wind (ZONE 3) In: ".to_string()
                + &convert_to_time_relative_string(current_time, wind_time)
                + " ("
                + &convert_to_time_string(wind_time)
                + ") "),
        );
    }

    let (is_thunder_active, thunder_time) = find_bsf_thunder(current_time);
    if is_thunder_active {
        return_str.push_str(
            &("\n - Thunder (ZONE 1) Now! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, thunder_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Thunder (ZONE 1) In: ".to_string()
                + &convert_to_time_relative_string(current_time, thunder_time)
                + " ("
                + &convert_to_time_string(thunder_time)
                + ") "),
        );
    }

    // Zadnor Weather
    return_str.push_str("\n\n-- Zadnor --");

    let (is_snow_active, snow_time) = find_zadnor_snow(current_time);
    if is_snow_active {
        return_str.push_str(
            &("\n - Snow (ZONE 3) Now! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, snow_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Snow (ZONE 3) In: ".to_string()
                + &convert_to_time_relative_string(current_time, snow_time)
                + " ("
                + &convert_to_time_string(snow_time)
                + ") "),
        );
    }

    let (is_zadnor_wind_active, zadnor_wind_time) = find_zadnor_wind(current_time);
    if is_zadnor_wind_active {
        return_str.push_str(
            &("\n - Wind (ZONE 1) Now! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, zadnor_wind_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Wind (ZONE 1) In: ".to_string()
                + &convert_to_time_relative_string(current_time, zadnor_wind_time)
                + " ("
                + &convert_to_time_string(zadnor_wind_time)
                + ") "),
        );
    }

    let (is_rain_active, rain_time) = find_zadnor_rain(current_time);
    if is_rain_active {
        return_str.push_str(
            &("\n - Rain (ZONE 2) Now! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, rain_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Rain (ZONE 2) In: ".to_string()
                + &convert_to_time_relative_string(current_time, rain_time)
                + " ("
                + &convert_to_time_string(rain_time)
                + ") "),
        );
    }

    let (is_zadnor_thunder_active, zadnor_thunder_time) = find_zadnor_thunder(current_time);
    if is_zadnor_thunder_active {
        return_str.push_str(
            &("\n - Thunder (ZONE 2) Now! Ends in ".to_string()
                + &convert_to_time_relative_string(current_time, zadnor_thunder_time)),
        );
    } else {
        return_str.push_str(
            &("\n - Thunder (ZONE 2) In: ".to_string()
                + &convert_to_time_relative_string(current_time, zadnor_thunder_time)
                + " ("
                + &convert_to_time_string(zadnor_thunder_time)
                + ") "),
        );
    }

    return_str
}

// Bozjan Southern Front
pub fn find_bsf_dust(current_time: i64) -> (bool, i64) {
    let bsf_weather = get_weather_by_time("Bozjan Southern Front", current_time);
    if bsf_weather == Weather::DustStorms {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_weather =
            find_next_weather_occurance("Bozjan Southern Front", current_time, Weather::DustStorms);
        (false, next_weather.start_time)
    }
}

pub fn find_bsf_wind(current_time: i64) -> (bool, i64) {
    let bsf_weather = get_weather_by_time("Bozjan Southern Front", current_time);
    if bsf_weather == Weather::Wind {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_weather =
            find_next_weather_occurance("Bozjan Southern Front", current_time, Weather::Wind);
        (false, next_weather.start_time)
    }
}

pub fn find_bsf_thunder(current_time: i64) -> (bool, i64) {
    let bsf_weather = get_weather_by_time("Bozjan Southern Front", current_time);
    if bsf_weather == Weather::Thunder {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_weather =
            find_next_weather_occurance("Bozjan Southern Front", current_time, Weather::Thunder);
        (false, next_weather.start_time)
    }
}

// Zadnor
pub fn find_zadnor_snow(current_time: i64) -> (bool, i64) {
    let zadnor_weather = get_weather_by_time("Zadnor", current_time);
    if zadnor_weather == Weather::Snow {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_weather = find_next_weather_occurance("Zadnor", current_time, Weather::Snow);
        (false, next_weather.start_time)
    }
}

pub fn find_zadnor_wind(current_time: i64) -> (bool, i64) {
    let zadnor_weather = get_weather_by_time("Zadnor", current_time);
    if zadnor_weather == Weather::Wind {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_weather = find_next_weather_occurance("Zadnor", current_time, Weather::Wind);
        (false, next_weather.start_time)
    }
}

pub fn find_zadnor_rain(current_time: i64) -> (bool, i64) {
    let zadnor_weather = get_weather_by_time("Zadnor", current_time);
    if zadnor_weather == Weather::Rain {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_weather = find_next_weather_occurance("Zadnor", current_time, Weather::Rain);
        (false, next_weather.start_time)
    }
}

pub fn find_zadnor_thunder(current_time: i64) -> (bool, i64) {
    let zadnor_weather = get_weather_by_time("Zadnor", current_time);
    if zadnor_weather == Weather::Thunder {
        let time_until_next_cycle = calculate_current_weather_interval(current_time);
        (true, time_until_next_cycle.1)
    } else {
        let next_weather = find_next_weather_occurance("Zadnor", current_time, Weather::Thunder);
        (false, next_weather.start_time)
    }
}
