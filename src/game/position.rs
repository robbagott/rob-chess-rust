use super::chess_move::ChessMove;
use super::game_piece::{GamePiece, Piece};
use super::side::Side;
use std::fmt;

// Square represents a square in a chess position. Squares can have a piece placed on them.
#[derive(Debug)]
pub struct Square {
    file: i32,
    rank: i32,
}

// Position represents a chess position representation.
#[derive(Debug)]
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

    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), ()> {
        let or = chess_move.o_rank;
        let of = chess_move.o_file;
        let nr = chess_move.n_rank;
        let nf = chess_move.n_file;

        if of > 7 || or > 7 || nf > 7 || nr > 7 {
            Err(())
        } else {
            // TODO handle promotion
            // Make normal move
            let piece = self.board[or][of];
            self.board[or][of] = GamePiece::new(Piece::None, Side::White);
            self.board[nr][nf] = piece;
            Ok(())
        }
    }

    // // MakeMove modifies the given position to represent the position after the move is made.
    // func (p *Position) MakeMove(move Move) bool {
    //     of, or := move.oFile, move.oRank
    //     nf, nr := move.nFile, move.nRank
    //     if of > 7 || of < 0 || or > 7 || or < 0 ||
    //         nf > 7 || nf < 0 || nr > 7 || nr < 0 {
    //         return false
    //     }

    //     // TODO handle promotion

    //     // Make normal move
    //     piece := p.board[or][of]
    //     p.board[or][of] = GamePiece{None, White}
    //     p.board[nr][nf] = piece
    //     return true
    // }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_print = String::new();
        board_print += "   ––––––––––––––––-----------------\n";
        for r in (0..self.board.len()).rev() {
            board_print += &format!(" {} ", (r as i32 + 1));
            for (f, _) in self.board[r].iter().enumerate() {
                let game_piece = self.board[r][f];
                board_print += &format!("| {} ", game_piece);
                if f == 7 {
                    board_print += "|\n   ––––––––––––––––-----------------\n";
                }
            }
        }
        board_print += "     a   b   c   d   e   f   g   h\n";
        return write!(f, "{}", board_print);
    }
}
