use std::error::Error;
use std::fs::File;

use serde_derive::Deserialize;

pub fn parse_config(file_name: &str) -> Result<Config, Box<dyn Error>> {
    let file = File::open(file_name)?;
    return Ok(serde_json::from_reader(file)?);
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub board_size: usize,
    pub required_icons_in_a_row: usize,
    pub player_num: usize,
}
