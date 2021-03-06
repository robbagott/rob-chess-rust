use super::color::Color;

use colored::*;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

// Pawn, Rook, Knight, Bishop, Queen, and King are the values for a piece. None is provided for empty squares.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
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
        };

        write!(f, "{}", piece_str)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ParseError {}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse string into Piece")
    }
}

impl FromStr for Piece {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P" => return Ok(Piece::Pawn),
            "R" => return Ok(Piece::Rook),
            "N" => return Ok(Piece::Knight),
            "b" => return Ok(Piece::Bishop),
            "Q" => return Ok(Piece::Queen),
            "K" => return Ok(Piece::King),
            _ => return Err(ParseError {}),
        };
    }
}

// GamePiece represents a piece in a chess game. E.g. a black bishop.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GamePiece {
    pub piece: Piece,
    pub color: Color,
}

impl GamePiece {
    pub fn new(piece: Piece, color: Color) -> GamePiece {
        GamePiece { piece, color }
    }

    pub fn value(&self) -> i32 {
        self.piece.value()
    }
}

impl fmt::Display for GamePiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_str = format!("{}", self.piece);

        if self.color == Color::White {
            return write!(f, "{}", piece_str.green());
        }
        write!(f, "{}", piece_str.red())
    }
}
