use super::side::Side;

use colored::*;
use std::fmt;

// Pawn, Rook, Knight, Bishop, Queen, and King are the values for a piece. None is provided for empty squares.
#[derive(Copy, Clone, Debug)]
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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_str = match self {
            Piece::Pawn => "P",
            Piece::Rook => "R",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Queen => "Q",
            Piece::King => "K",
            _ => " ",
        };

        write!(f, "{}", piece_str)
    }
}

// GamePiece represents a piece in a chess game. E.g. a black bishop.
#[derive(Copy, Clone, Debug)]
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
        let piece_str = format!("{}", self.piece);

        if self.color == Side::White {
            return write!(f, "{}", piece_str.green());
        }
        write!(f, "{}", piece_str.red())
    }
}
