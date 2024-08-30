use clap::{Parser, Arg, Command};
use chrono::Utc;

pub mod zone_weather_timer;
use zone_weather_timer::{handle_eureka_arisu, handle_bozja_arisu};

#[derive(Parser)]
#[command(name = "Arisu")]
#[command(about = "A CLI tool to help track certain things in FFXIV", long_about = None)]
struct Cli {
    pattern: String,
    #[arg(short, long)]
    options: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    
    let current_time_seconds = Utc::now().timestamp();
    let result = match args.pattern.as_str() {
        "eureka" => handle_eureka_arisu(current_time_seconds),
        "bozja" => handle_bozja_arisu(current_time_seconds),
        _ => {
            println!("Unknown pattern. Run with --help for more information");
            return;
        },
    };
    println!("{}", result);
}