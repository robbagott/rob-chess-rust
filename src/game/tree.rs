use super::chess_move::ChessMove;

#[derive(Debug)]
pub struct Tree {
    pub children: Vec<Tree>,
    pub chess_move: Option<ChessMove>,
    pub eval: Option<f32>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            children: Vec::<Tree>::new(),
            chess_move: None,
            eval: None,
        }
    }
}
