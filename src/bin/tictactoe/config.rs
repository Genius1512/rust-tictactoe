use std::error;

use clap::{value_parser, Arg, ArgAction, Command};
use tictactoe::{builtin::HumanPlayer, Error, Player};

pub fn parse_config() -> Result<Config, Box<dyn error::Error>> {
    let matches = Command::new("TicTacToe")
        .version("0.1.0")
        .author("Silvan Schmidt")
        .about("A customizable TicTacToe game")
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .default_value("3")
                .value_parser(value_parser!(usize))
                .help("The size of the board"),
        )
        .arg(
            Arg::new("win_condition")
                .short('w')
                .long("wincondition")
                .default_value("3")
                .value_parser(value_parser!(usize))
                .help("The amount of required icons in a row to win"),
        )
        .arg(
            Arg::new("human")
                .long("hp")
                .value_parser(value_parser!(char))
                .action(ArgAction::Append)
                .help("Add a player to the game. Specify the icon after the argument."),
        )
        .get_matches();

    let mut players: Vec<Box<dyn Player>> = vec![];

    for player in match matches.get_many::<char>("human") {
        Some(player) => player,
        None => return Err(Box::new(Error::new("Error while parsing players"))),
    } {
        players.push(Box::new(HumanPlayer::new(*player)))
    }

    Ok(Config {
        board_size: match matches.get_one("size") {
            Some(size) => *size,
            None => return Err(Box::new(Error::new("Error while parsing size"))),
        },
        required_icons_in_a_row: match matches.get_one("win_condition") {
            Some(i) => *i,
            None => return Err(Box::new(Error::new("Error while parsing win_condition"))),
        },
        players,
    })
}

pub struct Config {
    pub board_size: usize,
    pub required_icons_in_a_row: usize,
    pub players: Vec<Box<dyn Player>>,
}
