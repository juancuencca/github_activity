mod utils;
mod error;

pub use error::*;

use std::{
    env,
    process,
};

fn main() {
    let username = utils::parse_args(env::args()).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        eprintln!("Usage:");
        eprintln!("  cargo run -- <username>");
        process::exit(1);
    });

    println!("Username: {}", username);
}
