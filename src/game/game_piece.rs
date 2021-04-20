use super::side::Side;

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

// func (p GamePiece) String() string {
// 	var pieceStr string
// 	switch p.piece {
// 	case Pawn:
// 		pieceStr = "P"
// 	case Rook:
// 		pieceStr = "R"
// 	case Knight:
// 		pieceStr = "N"
// 	case Bishop:
// 		pieceStr = "B"
// 	case Queen:
// 		pieceStr = "Q"
// 	case King:
// 		pieceStr = "K"
// 	default:
// 		pieceStr = " "
// 	}

// 	if p.color == White {
// 		return color.GreenString(pieceStr)
// 	}
// 	return color.RedString(pieceStr)
// }
