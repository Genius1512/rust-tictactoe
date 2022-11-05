use std::{error, fmt};

use colored::Colorize;

use crate::{utils, Board, Error, Player, PlayerIndex};

pub struct Game {
    pub board: Board,

    pub players: Vec<Box<dyn Player>>,
    pub num_of_players: usize,

    pub win_condition: usize,
}

impl Game {
    pub fn new(
        size: usize,
        win_condition: usize,
        players: Vec<Box<dyn Player>>,
    ) -> Result<Game, Box<dyn error::Error>> {
        if players.len() < 2 {
            return Err(Box::new(Error::new("At least 2 players are required")));
        }

        if win_condition > size {
            return Err(Box::new(Error::new("Win condition is impossible to meet")));
        }

        let mut taken_icons: Vec<char> = vec![];
        for player in &players {
            if taken_icons.contains(&player.icon()) {
                return Err(Box::new(Error::new(&format!(
                    "The icon '{}' is used multiple times",
                    player.icon()
                ))));
            } else {
                taken_icons.push(player.icon());
            }
        }

        Ok(Game {
            board: Board::new(size)?,

            num_of_players: players.len(),
            players,

            win_condition,
        })
    }

    pub fn make_move(&mut self, player_index: PlayerIndex) -> Result<(), Box<dyn error::Error>> {
        let move_ = match self.players.get(player_index) {
            Some(player) => player.get_move(self),
            None => {
                return Err(Box::new(Error::new(&format!(
                    "Player with index {} does not exist",
                    player_index
                ))))
            }
        };

        self.board.make_move(move_, player_index)?;

        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::from("  ");

        for x in 0..self.board.size {
            let num = (x + 1).to_string();
            match num.len() {
                1 => out.push_str(&format!("{}  ", num)),
                2 => out.push_str(&format!("{} ", num)),
                _ => unreachable!(),
            }
        }
        out.push('\n');

        for x in 0..self.board.size {
            out.push(utils::digit_to_char(x + 1));
            out.push(' ');

            for y in 0..self.board.size {
                if self.board.move_history.len() != 0 {
                    let mut is_colored = false;

                    if x == self.board.move_history.last().unwrap().x
                        && y == self.board.move_history.last().unwrap().y
                    {
                        is_colored = true;
                    }

                    out.push_str(&match self.board[x][y] {
                        Some(player_index) => {
                            let mut text = self.players[player_index].icon().to_string();

                            if is_colored {
                                text = text.red().to_string();
                            }

                            text
                        }
                        None => " ".to_string(),
                    });

                    out.push_str("  ");
                } else {
                    out.push_str("   ");
                }
            }

            out.push('\n');
        }

        write!(f, "{}", out)
    }
}
