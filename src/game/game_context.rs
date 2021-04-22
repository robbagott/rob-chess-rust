use super::chess_move::ChessMove;
use super::position::Position;
use super::tree::Tree;

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

    // MakeMove makes a move in the game and records it.
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), ()> {
        self.position.make_move(chess_move)

        // if ok := g.position.MakeMove(move); ok {
        //     // Add move
        //     g.moves = append(g.moves, move)

        //     // Throw away unused game tree.
        //     for _, child := range g.gameTree.children {
        //         if child.move == move {
        //             g.gameTree = child
        //             break
        //         }
        //     }
        //     return true
        // }
        // return false
    }
}
