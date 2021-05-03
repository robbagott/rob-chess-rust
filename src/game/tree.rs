use super::chess_move::ChessMove;

#[derive(Debug)]
pub struct Tree {
    pub children: Vec<Node>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            children: Vec::<Node>::new(),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub chess_move: ChessMove,
    pub eval: Option<f64>,
}

impl Node {
    pub fn new(chess_move: ChessMove, eval: Option<f64>) -> Node {
        Node {
            children: Vec::<Node>::new(),
            chess_move,
            eval,
        }
    }
}
