// Pawn, Rook, Knight, Bishop, Queen, and King are the values for a piece. None is provided for empty squares.
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    None,
}

// White and Black are the values for a piece color.
pub enum Side {
    White,
    Black,
}

impl Side {
    // Return the opposite side to the one given.
    pub fn opp_side(s: Side) -> Side {
        match s {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

// GamePiece represents a piece in a chess game. E.g. a black bishop.
pub struct GamePiece {
    piece: Piece,
    color: Side,
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

// Value returns the value of the piece. For now, the value is the classical chess piece value.
fn value(p: GamePiece) -> i32 {
    match p.piece {
        Piece::Pawn => 1,
        Piece::Rook => 5,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Queen => 9,
        Piece::King => 200,
        _ => 1,
    }
}
