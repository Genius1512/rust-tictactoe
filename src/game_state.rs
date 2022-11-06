use crate::PlayerIndex;

pub enum GameState {
    Winner(PlayerIndex),
    Tie,
    None,
}
