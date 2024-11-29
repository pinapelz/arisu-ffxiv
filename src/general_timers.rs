use super::time_utils::{format_unix_to_date, format_unix_to_relative_timestr};
use chrono::{TimeZone, Utc};

pub fn get_next_gold_saucer_gate(current_time: i64) -> (bool, i64, String) {
    let minutes = current_time % 3600 / 60;
    let next_gate_time = if minutes < 10 {
        current_time - minutes * 60 + 600
    } else if minutes < 30 {
        current_time - (minutes - 20) * 60 + 600
    } else if minutes < 50 {
        current_time - (minutes - 40) * 60 + 600
    } else {
        current_time - (minutes - 60) * 60 + 600
    };
    let is_gate = (minutes % 20) < 10;

    // if not then get the next gate time
    if !is_gate {
        return (is_gate, next_gate_time, "Upcoming GATE".to_string());
    }
    let gate_name = if is_gate {
        "Possibly Active Gate".to_string()
    } else {
        "No Gate".to_string()
    };
    (is_gate, 0, gate_name)
}

pub fn handle_gate_message(current_time: i64) -> String {
    let (is_gate, next_gate_time, gate_name) = get_next_gold_saucer_gate(current_time);
    if is_gate {
        return "There is a GATE potentially now".to_string();
    }
    format!(
        "Next Gate is {} with the name {}",
        format_unix_to_relative_timestr(next_gate_time),
        gate_name
    )
}
