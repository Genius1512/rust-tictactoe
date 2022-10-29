use crate::Game;

pub trait Player {
    fn get_move(&self, game: &Game) -> (usize, usize);

    fn icon(&self) -> char;
}
