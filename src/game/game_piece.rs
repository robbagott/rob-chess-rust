use super::side::Side;

use colored::*;
use std::fmt;

// Pawn, Rook, Knight, Bishop, Queen, and King are the values for a piece. None is provided for empty squares.
#[derive(Copy, Clone)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    None,
}

impl Piece {
    // Value returns the value of the piece. For now, the value is the classical chess piece value.
    pub fn value(&self) -> i32 {
        match self {
            Piece::Pawn => 1,
            Piece::Rook => 5,
            Piece::Knight => 3,
            Piece::Bishop => 3,
            Piece::Queen => 9,
            Piece::King => 200,
            _ => 1,
        }
    }
}

// GamePiece represents a piece in a chess game. E.g. a black bishop.
#[derive(Copy, Clone)]
pub struct GamePiece {
    piece: Piece,
    color: Side,
}

impl GamePiece {
    pub fn new(piece: Piece, color: Side) -> GamePiece {
        GamePiece { piece, color }
    }

    pub fn value(&self) -> i32 {
        self.piece.value()
    }
}

impl fmt::Display for GamePiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut piece_str = "";
        match self.piece {
            Piece::Pawn => piece_str = "P",
            Piece::Rook => piece_str = "R",
            Piece::Knight => piece_str = "N",
            Piece::Bishop => piece_str = "B",
            Piece::Queen => piece_str = "Q",
            Piece::King => piece_str = "K",
            _ => piece_str = " ",
        }

        if self.color == Side::White {
            return write!(f, "{}", piece_str.green());
        }
        return write!(f, "{}", piece_str.red());
    }
}
