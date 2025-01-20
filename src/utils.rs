use crate::error::Result;
use crate::Event;
use reqwest::StatusCode;

pub fn parse_args(mut args: impl Iterator<Item=String>) -> Result<String> {
    args.next();
    match args.next() {
        Some(username) => Ok(username),
        None => Err("Username was not provided".into()),
    }
}

pub fn get_events(api_url: &str) -> Result<Vec<Event>> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(api_url).header("User-Agent", "rust-reqwest").send()?;

    match res.status() {
        StatusCode::OK => Ok(res.json::<Vec<Event>>()?),
        StatusCode::NOT_FOUND => Err("User not found".into()),
        StatusCode::BAD_REQUEST => Err("Bad request".into()),
        StatusCode::INTERNAL_SERVER_ERROR => Err("Internal server error".into()),
        status => Err(format!("Unexpected error occurred (status {})", status).into()),
    }
}