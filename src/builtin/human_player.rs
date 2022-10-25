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
    fn get_move(&self, _board: &Vec<Vec<Option<usize>>>) -> (usize, usize) {
        let mut i: usize;
        let mut j: usize;

        loop {
            print!("Enter your move: ");
            let input: String = read!();

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
            };

            j = match right.parse::<usize>() {
                Ok(j) => j,
                Err(_) => {
                    println!("Could not parse number");
                    continue;
                }
            };

            if i > 26 || j > 26 {
                println!("A number is too big; Cannot be more than 26");
                continue;
            };

            break;
        }

        return (i - 1, j - 1);
    }

    fn icon(&self) -> char {
        self.icon
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
