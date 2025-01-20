use std::{env, process};
use github_activity::utils;

fn main() {
    let username = utils::parse_args(env::args()).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        eprintln!("Usage:");
        eprintln!("$ cargo run -- <username>");
        process::exit(1);
    });

    let api_url = format!("https://api.github.com/users/{}/events", username); 

    let events = utils::get_events(&api_url).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    });

    for event in &events {
        println!("{}", event);
    }
}
