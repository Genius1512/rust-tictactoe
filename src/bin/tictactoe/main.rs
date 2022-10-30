use tictactoe::{Game, GameState};

mod config;
use config::parse_config;

fn main() {
    let config = match parse_config() {
        Ok(config) => config,
        Err(e) => panic!("Error: {}", e),
    };

    let mut game: Game = match Game::new(
        config.board_size,
        config.players,
        config.required_icons_in_a_row,
    ) {
        Ok(game) => game,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    'mainloop: loop {
        for player_index in 0..game.players.len() {
            println!("{}", game);
            println!("It's {}'s move!", game.players[player_index].icon());

            match game.make_move(player_index) {
                Ok(_) => {}
                Err(e) => println!("An error occured: {}", e),
            }

            match game.check_for_winner() {
                GameState::Winner(winner) => {
                    println!("{} won! Congratulations!", game.players[winner].icon());
                    break 'mainloop;
                }
                GameState::Tie => {
                    println!("It's a tie!");
                    break 'mainloop;
                }
                GameState::None => {}
            }
        }
    }
}
