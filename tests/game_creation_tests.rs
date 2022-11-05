use tictactoe::{players::RandomPlayer, Game};

#[test]
fn max_size_test() {
    match Game::new(
        30,
        3,
        vec![
            Box::new(RandomPlayer::new('x')),
            Box::new(RandomPlayer::new('o')),
        ],
    ) {
        Ok(_) => panic!("This should've caused an error, as the board size is over 26"),
        Err(_) => {}
    }
}

#[test]
fn win_condition_too_high_test() {
    match Game::new(
        3,
        10,
        vec![
            Box::new(RandomPlayer::new('x')),
            Box::new(RandomPlayer::new('o')),
        ],
    ) {
        Ok(_) => panic!(
            "This should've caused an error, as the win condition was higher than the board size"
        ),
        Err(_) => {}
    }
}

#[test]
fn not_enough_players_test() {
    match Game::new(3, 3, vec![Box::new(RandomPlayer::new('o'))]) {
        Ok(_) => panic!("This should've caused an error, as there are not enough players"),
        Err(_) => {}
    }
}

#[test]
fn same_icon_test() {
    match Game::new(
        3,
        3,
        vec![
            Box::new(RandomPlayer::new('x')),
            Box::new(RandomPlayer::new('x')),
        ],
    ) {
        Ok(_) => {
            panic!("This should've caused an error, as there are two players with the same icons")
        }
        Err(_) => {}
    }
}
