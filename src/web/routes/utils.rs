use chrono::{NaiveDateTime, TimeZone, Utc};

pub fn format_unix_to_date(unix_timestamp: i64) -> String {
    let naive_datetime =
        NaiveDateTime::from_timestamp_opt(unix_timestamp, 0).expect("Invalid Unix timestamp");
    let formatted_date = Utc
        .from_utc_datetime(&naive_datetime)
        .format("%Y-%m-%d %H:%M:%S");
    formatted_date.to_string()
}
