use std::error;
use std::fmt;

use crate::{error::Error, utils, GameState, Player};

use colored::Colorize;

pub struct Game {
    pub board: Vec<Vec<Option<usize>>>,
    pub board_size: usize,

    pub required_icons_in_a_row: usize,

    pub players: Vec<Box<dyn Player>>,

    //                     i      j      player
    pub move_history: Vec<(usize, usize, usize)>,
}

impl Game {
    pub fn new(
        size: usize,
        players: Vec<Box<dyn Player>>,
        required_icons_in_a_row: usize,
    ) -> Result<Game, Box<dyn error::Error>> {
        if size > 26 {
            return Err(Box::new(Error::new("Board is too big, maximum is 26")));
        }

        if required_icons_in_a_row > size {
            return Err(Box::new(Error::new(
                "This configuration is not allowed, as the required icons in a row must be below or equal to the board's size"
            )));
        }

        if players.len() < 2 {
            return Err(Box::new(Error::new(
                "Not enough players, at least two are required",
            )));
        }

        let mut board: Vec<Vec<Option<usize>>> = vec![];
        for i in 0..size {
            board.push(vec![]);
            for _ in 0..size {
                board[i].push(None);
            }
        }

        Ok(Game {
            board,
            board_size: size,

            required_icons_in_a_row,

            players,

            move_history: vec![],
        })
    }

    pub fn check_for_winner(&self) -> GameState {
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                // If no one is placed there ...
                if let None = self.board[i][j] {
                    continue; // ...skip to next field
                }

                // board[i][j] is used by a player

                let mut is_horizontal = true;
                let mut is_diagonal_right = true;
                let mut is_diagonal_left = true;
                let mut is_vertical = true;

                for offset in 1..self.required_icons_in_a_row {
                    // If i + j, j is not equal to the first field, it cannot be a winner
                    match self.board.get(i + offset) {
                        Some(row) => match row.get(j) {
                            Some(player) => match player {
                                Some(player_index) => {
                                    if *player_index != self.board[i][j].unwrap() {
                                        is_horizontal = false;
                                    }
                                }
                                None => is_horizontal = false, // board[i + offset][j] is None, cannot be winner
                            },
                            None => unreachable!(), // j is out of field, should not be reachable
                        },
                        None => is_horizontal = false, // i + offset is out of field
                    }

                    match self.board.get(i) {
                        Some(row) => match row.get(j + offset) {
                            Some(player) => match player {
                                Some(player_index) => {
                                    if *player_index != self.board[i][j].unwrap() {
                                        is_vertical = false;
                                    }
                                }
                                None => is_vertical = false, // board[i][j + offset] is None cannot be winner
                            },
                            None => is_vertical = false, // j + offset is out of field
                        },
                        None => unreachable!(), // i is out of field, should not be reachable
                    }

                    match self.board.get(i + offset) {
                        Some(row) => match row.get(j + offset) {
                            Some(player) => match player {
                                Some(player_index) => {
                                    if *player_index != self.board[i][j].unwrap() {
                                        is_diagonal_right = false;
                                    }
                                }
                                None => is_diagonal_right = false, // board[i + offset][j + offset] is None cannot be winner
                            },
                            None => is_diagonal_right = false, // j + offset is out of field
                        },
                        None => is_diagonal_right = false, // i + offset is out of field
                    }

                    if j < offset {
                        is_diagonal_left = false;
                        continue;
                    }

                    match self.board.get(i + offset) {
                        Some(row) => match row.get(j - offset) {
                            Some(player) => match player {
                                Some(player_index) => {
                                    if *player_index != self.board[i][j].unwrap() {
                                        is_diagonal_left = false;
                                    }
                                }
                                None => is_diagonal_left = false,
                            },
                            None => is_diagonal_left = false,
                        },
                        None => is_diagonal_left = false,
                    }
                }

                if is_vertical || is_horizontal || is_diagonal_right || is_diagonal_left {
                    return GameState::Winner(self.board[i][j].unwrap());
                }
            }
        }

        let mut is_a_tie = true;
        'outer: for j in 0..self.board_size {
            for i in 0..self.board_size {
                if self.board[i][j] == None {
                    is_a_tie = false;
                    break 'outer;
                }
            }
        }

        return if is_a_tie {
            GameState::Tie
        } else {
            GameState::None
        };
    }

    pub fn make_move(&mut self, player_index: usize) -> Result<(), Box<dyn error::Error>> {
        let (i, j) = match self.players.get(player_index) {
            Some(p) => p.get_move(&self),
            None => return Err(Box::new(Error::new("Player index out of range"))),
        };

        self.board[i][j] = Some(player_index);
        self.move_history.push((i, j, player_index));

        Ok(())
    }

    pub fn make_moves(
        &mut self,
        moves: Vec<(usize, usize, usize)>,
    ) -> Result<(), Box<dyn error::Error>> {
        for move_ in moves {
            self.board[move_.0][move_.1] = Some(move_.2);
        }

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
                if self.move_history.len() != 0 {
                    let mut is_colored = false;

                    if i == self.move_history.last().unwrap().0
                        && j == self.move_history.last().unwrap().1
                    {
                        is_colored = true;
                    }

                    out.push_str(&match self.board[i][j] {
                        Some(p_index) => {
                            if !is_colored {
                                self.players[p_index].icon().to_string()
                            } else {
                                self.players[p_index].icon().to_string().red().to_string()
                            }
                        }
                        None => " ".to_string(),
                    });

                    out.push_str("  ");
                } else {
                    out.push_str("   ");
                }
            }
            out.push_str("\n");
        }

        write!(f, "{}", out)
    }
}
