use chrono::Utc;
use clap::Parser;

pub mod zone_weather_timer;
use zone_weather_timer::{handle_bozja_arisu, handle_eureka_arisu};

mod general_timers;
use general_timers::handle_gate_message;

mod time_utils;
mod web;

#[derive(Parser)]
#[command(name = "Arisu")]
#[command(about = "A CLI tool to help track certain things in FFXIV", long_about = None)]
struct Cli {
    #[arg()]
    pattern: String,
    #[arg(short, long)]
    options: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let current_time_seconds = Utc::now().timestamp();
    match args.pattern.as_str() {
        "eureka" => {
            println!("{}", handle_eureka_arisu(current_time_seconds));
        }
        "bozja" => {
            println!("{}", handle_bozja_arisu(current_time_seconds));
        }
        "gate" => {
            println!("{}", handle_gate_message(current_time_seconds));
        }
        "web" => {
            web::start_web_server().await;
        }
        _ => {
            println!("Unknown pattern. Run with --help for more information");
        }
    };
}
