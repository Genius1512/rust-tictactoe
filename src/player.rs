use crate::{Game, Move};

pub trait Player {
    fn get_move(&self, game: &Game) -> Move;

    fn icon(&self) -> char;
}

pub type PlayerIndex = usize;
