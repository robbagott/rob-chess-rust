use super::chess_move::ChessMove;
use super::color::Color;
use super::game_context::GameContext;

static THINK_DEPTH: u32 = 3;

pub fn think(g: &GameContext, color: Color) -> ChessMove {
    // Get an initial move
    let strongest_move = think_depth(g, color, THINK_DEPTH);
    strongest_move
}

pub fn think_depth(g: &GameContext, color: Color, depth: u32) -> ChessMove {
    let p = &g.position;

    // Check if there are moves on the node. If not, retrieve them and add them to the node.
    if g.tree.children.len() == 0 {
        let moves = p.get_moves(color);
        for chess_move in moves {}
    }

    ChessMove::from_algebraic("d7d5").unwrap()
}
