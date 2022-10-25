use std::{error::Error, fmt};

#[derive(Debug)]
pub struct TicTacToeError {
    msg: String,
}

impl TicTacToeError {
    pub fn new(msg: &str) -> TicTacToeError {
        TicTacToeError {
            msg: msg.to_string(),
        }
    }
}

impl Error for TicTacToeError {}

impl fmt::Display for TicTacToeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.msg)
    }
}
