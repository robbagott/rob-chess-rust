use super::game_piece;

// Square represents a square in a chess position. Squares can have a piece placed on them.
pub struct Square {
    file: i32,
    rank: i32,
}

// Position represents a chess position representation.
pub struct Position {
    board: [[game_piece::GamePiece; 8]; 8],
    o_o_o_white: bool,
    o_o_white: bool,
    o_o_o_black: bool,
    o_o_black: bool,
}
