use super::chess_move::ChessMove;

use super::position::Position;
use super::tree::Tree;
use std::mem;

#[derive(Debug)]
pub struct GameContext {
    pub position: Position,
    pub chess_moves: Vec<ChessMove>,
    pub tree: Tree,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            position: Position::new(),
            chess_moves: Vec::<ChessMove>::new(),
            tree: Tree::new(),
        }
    }

    // Makes a move in the game and records it.
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), ()> {
        self.position.make_move(&chess_move)?;
        self.chess_moves.push(chess_move);

        //println!("{:?}", self.tree.children);
        // Discard unused parts of the tree.
        let children = &mut self.tree.children;
        for node in children.into_iter() {
            if node.chess_move == chess_move {
                self.tree.children = mem::replace(&mut node.children, vec![]);
                break;
            }
        }

        return Ok(());
    }
}
