use tictactoe::{builtin::HumanPlayer, Game};

fn main() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x', "Player One")),
            Box::new(HumanPlayer::new('o', "Player Two")),
        ],
        3,
    );

    game.make_move(0).unwrap();

    println!("{}", game);

    game.make_move(1).unwrap();

    println!("{}", game);
}
