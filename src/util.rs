use std::fs::read_to_string;
use std::io::Error;

pub(crate) fn get_credentials() -> Result<(String, String), Error> {
    let txt = read_to_string(".discord-tokens")?;
    let tokens: Vec<&str> = txt.split("\n").collect();

    Ok((tokens[0].to_string(), tokens[1].to_string()))
}