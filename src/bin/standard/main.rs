use tictactoe::{builtin::HumanPlayer, Game};

fn main() {
    let mut game = Game::new(
        16,
        vec![
            Box::new(HumanPlayer::new('x', "Player One")),
            Box::new(HumanPlayer::new('o', "Player Two")),
            Box::new(HumanPlayer::new('!', "Player Three")),
        ],
        5,
    );

    'mainloop: loop {
        for player_index in 0..game.players.len() {
            println!("{}", game);
            println!("It's {}'s move!", game.players[player_index].name());

            match game.make_move(player_index) {
                Ok(_) => {}
                Err(e) => println!("An error occured: {}", e),
            }

            match game.check_for_winner() {
                Some(winner) => {
                    println!("{} won! Congratulations!", game.players[winner].name());
                    break 'mainloop;
                }
                None => {}
            }
        }
    }
}
