use tictactoe::builtin::HumanPlayer;
use tictactoe::Game;

#[test]
fn check_for_winner_diagonal_left_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x')),
            Box::new(HumanPlayer::new('o')),
        ],
        3,
    );

    game.make_moves(vec![(0, 0, 0)]).unwrap();

    match game.check_for_winner() {
        tictactoe::GameState::None => {}
        _ => panic!("Should not have won"),
    }

    game.make_moves(vec![(0, 4, 0), (1, 3, 0), (2, 2, 0)])
        .unwrap();

    match game.check_for_winner() {
        tictactoe::GameState::Winner(winner) => assert_eq!(winner, 0),
        _ => panic!("Did not recognize win; 0 should have won"),
    }
}

#[test]
fn check_for_winner_diagonal_right_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x')),
            Box::new(HumanPlayer::new('o')),
        ],
        3,
    );

    game.make_moves(vec![(0, 0, 0), (1, 1, 0), (2, 2, 0)])
        .unwrap();

    match game.check_for_winner() {
        tictactoe::GameState::Winner(winner) => assert_eq!(winner, 0),
        _ => panic!("Did not recognize win; 0 should have won"),
    }
}

#[test]
fn check_for_winner_horizontal_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x')),
            Box::new(HumanPlayer::new('o')),
        ],
        3,
    );

    game.make_moves(vec![(0, 0, 0), (0, 1, 0), (0, 2, 0)])
        .unwrap();

    match game.check_for_winner() {
        tictactoe::GameState::Winner(winner) => assert_eq!(winner, 0),
        _ => panic!("Did not recognize win; 0 should have won"),
    }
}

#[test]
fn check_for_winner_vertical_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x')),
            Box::new(HumanPlayer::new('o')),
        ],
        3,
    );

    game.make_moves(vec![(0, 0, 0), (1, 0, 0), (2, 0, 0)])
        .unwrap();

    match game.check_for_winner() {
        tictactoe::GameState::Winner(winner) => assert_eq!(winner, 0),
        _ => panic!("Did not recognize win; 0 should have won"),
    }
}

#[test]
fn check_for_winner_tie_test() {
    let mut game = Game::new(
        3,
        vec![
            Box::new(HumanPlayer::new('x')),
            Box::new(HumanPlayer::new('o')),
        ],
        3,
    );

    game.make_moves(vec![
        (0, 0, 0),
        (0, 1, 1),
        (0, 2, 0),
        (1, 0, 0),
        (1, 1, 1),
        (1, 2, 0),
        (2, 0, 1),
        (2, 1, 0),
        (2, 2, 1),
    ])
    .unwrap();

    println!("{}", game);

    match game.check_for_winner() {
        tictactoe::GameState::Tie => {}
        _ => panic!("Did not recognize tie;"),
    }
}
