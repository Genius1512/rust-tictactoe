use crate::{Game, GameState};

pub trait Player {
    fn get_move(&self, game: &Game) -> (usize, usize);

    fn icon(&self) -> char;

    #[allow(unused_variables)]
    fn communicate_end_of_game(&self, state: GameState) {}
}
