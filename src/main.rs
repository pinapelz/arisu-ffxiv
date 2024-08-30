use clap::Parser;
use chrono::Utc;

pub mod zone_weather_timer;
use zone_weather_timer::{handle_eureka_arisu, handle_bozja_arisu};

#[derive(Parser)]
#[command(name = "Arisu")]
#[command(about = "A CLI tool to help track certain things in FFXIV")]
struct Cli {
    pattern: String,
    options: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    
    let current_time_seconds = Utc::now().timestamp();
    let result = match args.pattern.as_str() {
        "eureka" => handle_eureka_arisu(current_time_seconds),
        "bozja" => handle_bozja_arisu(current_time_seconds),
        "help" => {
            handle_help();
            return;
        },
        _ => "Unknown pattern. Run with --help for more information".to_string(),
    };
    println!("{}", result);
}


fn handle_help(){
    println!("Arisu is a CLI tool to help track certain things in FFXIV");
    println!("Usage: arisu [pattern] [options]");
    println!("Patterns:");
    println!("eureka: Track Eureka weather patterns");
    println!("bozja: Track Bozja weather patterns");
    println!("Options:");
    println!("--help: Display this help message");
}

