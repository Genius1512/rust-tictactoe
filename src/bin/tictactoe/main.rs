use tictactoe::{players::RandomPlayer, Game};

fn main() {
    let mut game = Game::new(
        5,
        3,
        vec![
            Box::new(RandomPlayer::new('x')),
            Box::new(RandomPlayer::new('o')),
        ],
    )
    .unwrap();

    game.make_move(0).unwrap();
    game.make_move(1).unwrap();

    println!("{}", game);
}
