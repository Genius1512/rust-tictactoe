use text_io::read;

use crate::{utils, Player};

pub struct HumanPlayer {
    icon: char,
    name: String,
}

impl HumanPlayer {
    pub fn new(icon: char, name: &str) -> HumanPlayer {
        HumanPlayer {
            icon,
            name: name.to_string(),
        }
    }
}

impl Player for HumanPlayer {
    fn get_move(&self, board: &Vec<Vec<Option<usize>>>) -> (usize, usize) {
        let mut i: usize;
        let mut j: usize;

        loop {
            print!("Enter your move: ");
            let input: String = read!();
            let input = input.to_lowercase();

            if input.matches('_').count() != 1 {
                println!("{}", input.matches('_').count());
                println!("None or too many spaces in input");
                continue;
            };

            let split: Vec<&str> = input.split('_').collect();
            let (left, right) = (split[0], split[1]);

            let left: char = match left.chars().next() {
                Some(c) => c,
                None => {
                    println!("Could not get first char");
                    continue;
                }
            };

            i = match utils::char_to_digit(left) {
                Ok(i) => i,
                Err(_) => {
                    println!("Could not convert char to digit");
                    continue;
                }
            } - 1;

            j = match right.parse::<usize>() {
                Ok(j) => j,
                Err(_) => {
                    println!("Could not parse number");
                    continue;
                }
            } - 1;

            if i > 26 || j > 26 {
                println!("A number is too big; Cannot be more than 26");
                continue;
            };

            match board[i][j] {
                Some(_) => {
                    println!("Field is already taken");
                    continue;
                }

                None => {}
            };

            break;
        }

        return (i, j);
    }

    fn icon(&self) -> char {
        self.icon
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
