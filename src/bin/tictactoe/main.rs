use tictactoe::Game;

fn main() {
    let mut game = Game::new(5, 3, vec![]).unwrap();

    println!("{}", game);
}
