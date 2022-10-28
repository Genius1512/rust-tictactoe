use tictactoe::{builtin::HumanPlayer, Game};

fn main() {
    let mut game = Game::new(
        16,
        vec![
            Box::new(HumanPlayer::new('x', "Player One")),
            Box::new(HumanPlayer::new('o', "Player Two")),
        ],
        5,
    );

    let mut player_index: usize = 0;

    loop {
        game.make_move(player_index).unwrap();

        if let Some(winner) = game.check_for_winner() {
            println!("{} won!", game.players[winner].name());
            break;
        }

        if player_index == 0 {
            player_index = 1;
        } else {
            player_index = 0;
        }
    }
}
