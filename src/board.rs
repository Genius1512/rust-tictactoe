use std::{error, ops::Deref};

use crate::{Error, Move, PlayerIndex};

pub type BoardType = Vec<Vec<Option<PlayerIndex>>>;

pub struct Board {
    pub board: BoardType,
    pub size: usize,

    pub move_history: Vec<Move>,
}

impl Board {
    pub fn new(size: usize) -> Result<Board, Box<dyn error::Error>> {
        if size > 26 {
            return Err(Box::new(Error::new("Board size can be 26 at maximum")));
        }

        let mut board: BoardType = vec![];

        for x in 0..size {
            board.push(vec![]);
            for _ in 0..size {
                board[x].push(None);
            }
        }

        Ok(Board {
            board,
            size,
            move_history: vec![],
        })
    }

    pub fn make_move(
        &mut self,
        move_: Move,
        player_index: PlayerIndex,
    ) -> Result<(), Box<dyn error::Error>> {
        if move_.x >= self.size || move_.y >= self.size {
            return Err(Box::new(Error::new(&format!(
                "Could not make move, an index is out of range. x={}, y={}, size={}",
                move_.x, move_.y, self.size,
            ))));
        }

        self.board[move_.x][move_.y] = Some(player_index);
        self.move_history.push(move_);

        Ok(())
    }
}

impl Deref for Board {
    type Target = BoardType;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}
