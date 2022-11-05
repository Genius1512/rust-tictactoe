use rand::Rng;

use crate::{Game, Move, Player};

pub struct RandomPlayer {
    icon: char,
}

impl RandomPlayer {
    pub fn new(icon: char) -> RandomPlayer {
        RandomPlayer { icon }
    }
}

impl Player for RandomPlayer {
    fn get_move(&self, game: &Game) -> Move {
        loop {
            let x = rand::thread_rng().gen_range(0..game.board.size);
            let y = rand::thread_rng().gen_range(0..game.board.size);

            match game.board[x][y] {
                Some(_) => continue,
                None => return Move { x, y },
            }
        }
    }

    fn icon(&self) -> char {
        self.icon
    }
}
