use tictactoe::{builtin::HumanPlayer, Game};

fn main() {
    let mut game = Game::default(
        Box::new(HumanPlayer::new('x', "Player One")),
        Box::new(HumanPlayer::new('o', "Player Two")),
    );

    loop {
        game.make_move(0).unwrap();
        game.make_move(1).unwrap();
    }
}
