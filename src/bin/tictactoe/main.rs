use tictactoe::{builtin::HumanPlayer, Game};

fn main() {
    let game = Game::default(
        Box::new(HumanPlayer::new('x', "Player One")),
        Box::new(HumanPlayer::new('o', "Player Two")),
    );
}
