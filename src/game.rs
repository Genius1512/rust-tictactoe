use std::error::Error;
use std::fmt;

use crate::{tictactoe_error::TicTacToeError, utils, GameState, Player};

pub struct Game {
    pub board: Vec<Vec<Option<usize>>>,
    board_size: usize,

    required_icons_in_a_row: usize,

    pub players: Vec<Box<dyn Player>>,
}

impl Game {
    pub fn new(size: usize, players: Vec<Box<dyn Player>>, required_icons_in_a_row: usize) -> Game {
        if size > 26 {
            panic!("Board is too big")
        }

        if required_icons_in_a_row > size {
            panic!("This configuration is not possible, as there is no constellation where someone wins");
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

    pub fn make_move(&mut self, player_index: usize) -> Result<(), Box<dyn Error>> {
        let (i, j) = match self.players.get(player_index) {
            Some(p) => p.get_move(&self),
            None => return Err(Box::new(TicTacToeError::new("Player index out of range"))),
        };

        self.board[i][j] = Some(player_index);

        Ok(())
    }

    pub fn make_moves(&mut self, moves: Vec<(usize, usize, usize)>) -> Result<(), Box<dyn Error>> {
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
