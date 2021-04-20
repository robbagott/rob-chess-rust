use super::game_piece::{GamePiece, Piece};
use super::side::Side;

// Square represents a square in a chess position. Squares can have a piece placed on them.
pub struct Square {
    file: i32,
    rank: i32,
}

// Position represents a chess position representation.
pub struct Position {
    board: [[GamePiece; 8]; 8],
    o_o_o_white: bool,
    o_o_white: bool,
    o_o_o_black: bool,
    o_o_black: bool,
}

impl Position {
    pub fn new() -> Position {
        let board = [[GamePiece::new(Piece::None, Side::White); 8]; 8];
        let mut pos = Position {
            board,
            o_o_o_white: true,
            o_o_white: true,
            o_o_o_black: true,
            o_o_black: true,
        };

        pos.reset();
        pos
    }

    pub fn reset(&mut self) {
        for r in 0..8 {
            for f in 0..8 {
                match r {
                    0 => match f {
                        0 | 7 => self.board[r][f] = GamePiece::new(Piece::Rook, Side::White),
                        1 | 6 => self.board[r][f] = GamePiece::new(Piece::Knight, Side::White),
                        2 | 5 => self.board[r][f] = GamePiece::new(Piece::Bishop, Side::White),
                        3 => self.board[r][f] = GamePiece::new(Piece::Queen, Side::White),
                        4 => self.board[r][f] = GamePiece::new(Piece::King, Side::White),
                        _ => (),
                    },
                    1 => self.board[r][f] = GamePiece::new(Piece::Pawn, Side::White),
                    2 | 3 | 4 | 5 => self.board[r][f] = GamePiece::new(Piece::None, Side::White),
                    6 => self.board[r][f] = GamePiece::new(Piece::Pawn, Side::Black),
                    7 => match f {
                        0 | 7 => self.board[r][f] = GamePiece::new(Piece::Rook, Side::Black),
                        1 | 6 => self.board[r][f] = GamePiece::new(Piece::Knight, Side::Black),
                        2 | 5 => self.board[r][f] = GamePiece::new(Piece::Bishop, Side::Black),
                        3 => self.board[r][f] = GamePiece::new(Piece::Queen, Side::Black),
                        4 => self.board[r][f] = GamePiece::new(Piece::King, Side::Black),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }
}
