use chrono::{TimeZone, Local, Utc};

pub fn convert_to_time_string(timestamp: i64) -> String {
    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
    let local_dt = dt.with_timezone(&Local);
    let dt_string = local_dt.format("%Y-%m-%d %H:%M:%S").to_string();
    dt_string
}

pub fn convert_to_time_relative_string(current_time: i64, future_timestamp: i64) -> String {
    let time_diff = future_timestamp - current_time;
    if time_diff <= 0 {
        return String::from("the past");
    }
    let hours = time_diff / 3600;
    let minutes = (time_diff % 3600) / 60;
    let mut relative_string = String::new();
    if hours > 0 {
        relative_string.push_str(&format!("{} hour{}", hours, if hours > 1 { "s" } else { "" }));
    }
    if hours > 0 && minutes > 0 {
        relative_string.push_str(" and ");
    }
    if minutes > 0 {
        relative_string.push_str(&format!("{} minute{}", minutes, if minutes > 1 { "s" } else { "" }));
    }
    relative_string
}