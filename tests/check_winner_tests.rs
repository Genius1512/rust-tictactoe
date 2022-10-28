use tictactoe::builtin::HumanPlayer;
use tictactoe::Game;

#[test]
fn check_for_winner_diagonal_left_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x', "Player One")),
            Box::new(HumanPlayer::new('o', "Player Two")),
        ],
        3,
    );

    game.make_moves(vec![(0, 4, 0), (1, 3, 0), (2, 2, 0)])
        .unwrap();

    match game.check_for_winner() {
        Some(winner) => assert_eq!(winner, 0),
        None => panic!("Did not recognize win; 0 should have won"),
    }
}

#[test]
fn check_for_winner_diagonal_right_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x', "Player One")),
            Box::new(HumanPlayer::new('o', "Player Two")),
        ],
        3,
    );

    game.make_moves(vec![(0, 0, 0), (1, 1, 0), (2, 2, 0)])
        .unwrap();

    match game.check_for_winner() {
        Some(winner) => assert_eq!(winner, 0),
        None => panic!("Did not recognize win; 0 should have won"),
    }
}

#[test]
fn check_for_winner_horizontal_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x', "Player One")),
            Box::new(HumanPlayer::new('o', "Player Two")),
        ],
        3,
    );

    game.make_moves(vec![(0, 0, 0), (0, 1, 0), (0, 2, 0)])
        .unwrap();

    match game.check_for_winner() {
        Some(winner) => assert_eq!(winner, 0),
        None => panic!("Did not recognize win; 0 should have won"),
    }
}

#[test]
fn check_for_winner_vertical_test() {
    let mut game = Game::new(
        5,
        vec![
            Box::new(HumanPlayer::new('x', "Player One")),
            Box::new(HumanPlayer::new('o', "Player Two")),
        ],
        3,
    );

    game.make_moves(vec![(0, 0, 0), (1, 0, 0), (2, 0, 0)])
        .unwrap();

    match game.check_for_winner() {
        Some(winner) => assert_eq!(winner, 0),
        None => panic!("Did not recognize win; 0 should have won"),
    }
}
