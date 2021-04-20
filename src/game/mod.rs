mod game_piece;
mod position;
mod side;

use crate::tree::Node;
use crate::tree::Tree;
use position::Position;
use regex::Regex;

struct GameContext {
    position: Position,
    chess_moves: Vec<ChessMove>,
    game_tree: Tree<Node<ChessMove>>,
}

impl GameContext {
    fn new() -> GameContext {
        GameContext {
            position: Position::new(),
            chess_moves: Vec::<ChessMove>::new(),
            game_tree: Tree::<Node<ChessMove>>::new(),
        }
    }
}

// Move represents a move on the chess board. It encompasses a piece, the old square and the new square.
struct ChessMove {
    o_file: i32,
    o_rank: i32,
    n_file: i32,
    n_rank: i32,
    promo_piece: String,
}

impl PartialEq for ChessMove {
    fn eq(&self, other: &Self) -> bool {
        self.o_file == other.o_file
            && self.o_rank == other.o_rank
            && self.n_file == other.n_file
            && self.n_rank == other.n_rank
            && self.promo_piece == other.promo_piece
    }
}

pub fn start_user_session() {
    let _re = Regex::new("(?P<file1>[a-h])(?P<rank1>[1-8])(?P<file2>[a-h])(?P<rank2>[1-8])(?P<promotionPiece>[bnrq])?").unwrap();
    println!(
        "Welcome to Rob Chess! When entering moves, please use long algebraic chess notation."
    );
}
