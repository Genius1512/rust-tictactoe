use tictactoe::{players::RandomPlayer, Game, Move};

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

    game.board
        .make_moves(vec![(Move { x: 0, y: 0 }, 0), (Move { x: 1, y: 1 }, 1)])
        .unwrap();

    println!("{}", game);
}
