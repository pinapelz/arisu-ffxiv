use chrono::{Local, NaiveDateTime, TimeZone};

pub fn format_unix_to_date(unix_timestamp: i64) -> String {
    let naive_datetime =
        NaiveDateTime::from_timestamp_opt(unix_timestamp, 0).expect("Invalid Unix timestamp");
    let formatted_date = Local
        .from_utc_datetime(&naive_datetime)
        .format("%Y-%m-%d %H:%M");
    formatted_date.to_string()
}

pub fn format_unix_to_relative_timestr(unix_timestamp: i64) -> String {
    let now = Local::now().timestamp();
    let duration = now - unix_timestamp;

    if duration == 0 {
        "right now".to_string()
    } else if duration > 0 {
        if duration < 60 {
            "less than a minute ago".to_string()
        } else if duration < 3600 {
            format!("{} minutes ago", duration / 60)
        } else if duration < 86400 {
            let hours = duration / 3600;
            let minutes = (duration % 3600) / 60;
            if minutes > 0 {
                format!("{} hours and {} minutes ago", hours, minutes)
            } else {
                format!("{} hours ago", hours)
            }
        } else {
            format!("{} days ago", duration / 86400)
        }
    } else {
        let future_duration = -duration;
        if future_duration < 60 {
            "in less than a minute".to_string()
        } else if future_duration < 3600 {
            format!("in {} minutes", future_duration / 60)
        } else if future_duration < 86400 {
            let hours = future_duration / 3600;
            let minutes = (future_duration % 3600) / 60;
            if minutes > 0 {
                format!("in {} hours and {} minutes", hours, minutes)
            } else {
                format!("in {} hours", hours)
            }
        } else {
            format!("in {} days", future_duration / 86400)
        }
    }
}
