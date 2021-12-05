use super::chess_move::ChessMove;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub last_move: Option<ChessMove>,
    pub eval: Option<f64>,
}

impl Node {
    pub fn new(last_move: Option<ChessMove>, eval: Option<f64>) -> Node {
        Node {
            children: Vec::<Node>::new(),
            last_move,
            eval,
        }
    }
}
