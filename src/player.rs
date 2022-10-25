pub trait Player {
    fn get_move(&self, board: &Vec<Vec<Option<usize>>>) -> (usize, usize);

    fn icon(&self) -> char;
    fn name(&self) -> String;
}
