use crate::error::Result;

pub fn parse_args(mut args: impl Iterator<Item=String>) -> Result<String> {
    args.next();
    match args.next() {
        Some(username) => Ok(username),
        None => Err("Username was not provided".into()),
    }
}