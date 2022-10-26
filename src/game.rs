use std::error::Error;
use std::fmt;

use crate::{tictactoe_error::TicTacToeError, utils, Player};

pub struct Game {
    pub board: Vec<Vec<Option<usize>>>,
    board_size: usize,

    #[allow(dead_code)]
    required_icons_in_a_row: usize,

    players: Vec<Box<dyn Player>>,
}

impl Game {
    pub fn new(size: usize, players: Vec<Box<dyn Player>>, required_icons_in_a_row: usize) -> Game {
        if size > 26 {
            panic!("Board is too big")
        }

        if players.len() < 2 {
            panic!("Not enought players. At least two are required");
        }

        let mut board: Vec<Vec<Option<usize>>> = vec![];
        for i in 0..size {
            board.push(vec![]);
            for _ in 0..size {
                board[i].push(None);
            }
        }

        Game {
            board,
            board_size: size,

            required_icons_in_a_row,

            players,
        }
    }

    pub fn check_for_winner(&self) -> Result<Option<usize>, Box<dyn Error>> {
        todo!()
    }

    pub fn make_move(&mut self, player_index: usize) -> Result<(), Box<dyn Error>> {
        let (i, j) = match self.players.get(player_index) {
            Some(p) => p.get_move(&self),
            None => return Err(Box::new(TicTacToeError::new("Player index out of range"))),
        };

        self.board[i][j] = Some(player_index);

        Ok(())
    }
}

/*
  1  2  3  4
a x
b
c    o
d          x
*/

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = String::new();
        out.push_str("  ");

        for i in 0..self.board_size {
            let num = (i + 1).to_string();
            match num.len() {
                1 => out.push_str(&format!("{}  ", num)),
                2 => out.push_str(&format!("{} ", num)),
                _ => unreachable!(),
            }
        }
        out.push('\n');

        for i in 0..self.board_size {
            out.push(utils::digit_to_char(i + 1));
            out.push(' ');

            for j in 0..self.board_size {
                out.push(match self.board[i][j] {
                    Some(p_index) => self.players[p_index].icon(),
                    None => ' ',
                });

                out.push_str("  ");
            }
            out.push_str("\n");
        }

        write!(f, "{}", out)
    }
}

/// Builtin variations
impl Game {
    pub fn default(player_one: Box<dyn Player>, player_two: Box<dyn Player>) -> Game {
        return Game::new(3, vec![player_one, player_two], 3);
    }

    pub fn default_three_players(
        player_one: Box<dyn Player>,
        player_two: Box<dyn Player>,
        player_three: Box<dyn Player>,
    ) -> Game {
        return Game::new(4, vec![player_one, player_two, player_three], 3);
    }
}
