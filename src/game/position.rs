use super::chess_move::ChessMove;
use super::color::Color;
use super::game_piece::{GamePiece, Piece};
use std::fmt;

// Square represents a square in a chess position. Squares can have a piece placed on them.
#[derive(Debug)]
pub struct Square {
    pub file: i32,
    pub rank: i32,
}

// Position represents a chess position representation.
#[derive(Debug)]
pub struct Position {
    pub board: [[Option<GamePiece>; 8]; 8],
    pub o_o_o_white: bool,
    pub o_o_white: bool,
    pub o_o_o_black: bool,
    pub o_o_black: bool,
}

impl Position {
    pub fn new() -> Position {
        let board = [[None; 8]; 8];
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
        for r in 0..self.board.len() {
            for f in 0..self.board[r].len() {
                match r {
                    0 => match f {
                        0 | 7 => self.board[r][f] = Some(GamePiece::new(Piece::Rook, Color::White)),
                        1 | 6 => {
                            self.board[r][f] = Some(GamePiece::new(Piece::Knight, Color::White))
                        }
                        2 | 5 => {
                            self.board[r][f] = Some(GamePiece::new(Piece::Bishop, Color::White))
                        }
                        3 => self.board[r][f] = Some(GamePiece::new(Piece::Queen, Color::White)),
                        4 => self.board[r][f] = Some(GamePiece::new(Piece::King, Color::White)),
                        _ => (),
                    },
                    1 => self.board[r][f] = Some(GamePiece::new(Piece::Pawn, Color::White)),
                    6 => self.board[r][f] = Some(GamePiece::new(Piece::Pawn, Color::Black)),
                    7 => match f {
                        0 | 7 => self.board[r][f] = Some(GamePiece::new(Piece::Rook, Color::Black)),
                        1 | 6 => {
                            self.board[r][f] = Some(GamePiece::new(Piece::Knight, Color::Black))
                        }
                        2 | 5 => {
                            self.board[r][f] = Some(GamePiece::new(Piece::Bishop, Color::Black))
                        }
                        3 => self.board[r][f] = Some(GamePiece::new(Piece::Queen, Color::Black)),
                        4 => self.board[r][f] = Some(GamePiece::new(Piece::King, Color::Black)),
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
            self.board[or][of] = None;
            self.board[nr][nf] = piece;
            Ok(())
        }
    }

    // TODO combine check pruning with get_moves_at for mem/cpu efficiency boost.
    // pub fn get_moves(&self, color: Color) -> Vec<ChessMove> {
    //     let moves = Vec::<ChessMove>::with_capacity(20);

    //     /* Only pieces can make moves in chess, so we iterate through the board and check for pieces.
    //     If a piece is found, we then check for legal moves for that piece. */
    //     for r in 0..self.board.len() {
    //         for f in 0..self.board[r].len() {
    //             if let Some(p) = self.board[r][f] {
    //                 if p.color == color {
    //                     moves.append(&mut self.get_moves_at(f, r, color));
    //                 }
    //             }
    //         }
    //     }

    //     // Prune moves which lead to checks
    //     let valid_moves = Vec::<ChessMove>::with_capacity(20);
    //     for m in moves.iter() {
    //         if self.causes_check(m, color) {
    //             valid_moves.push(m);
    //         }
    //     }

    //     valid_moves
    // }

    // fn get_moves_at(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
    //     let moves = Vec::<ChessMove>::with_capacity(20);
    //     let piece = self.board[r][f];
    // }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_print = String::new();
        board_print += "   ––––––––––––––––-----------------\n";
        for r in (0..self.board.len()).rev() {
            board_print += &format!(" {} ", (r as i32 + 1));
            for f in 0..self.board[r].len() {
                // let p_str = " ";
                // if let Some(p) = self.board[r][f] {
                //     p_str = format!("{}", p);
                // }

                let p_str = self.board[r][f]
                    .map(|p| format!("{}", p))
                    .unwrap_or(" ".to_string());

                board_print += &format!("| {} ", p_str);
                if f == 7 {
                    board_print += "|\n   ––––––––––––––––-----------------\n";
                }
            }
        }
        board_print += "     a   b   c   d   e   f   g   h\n";
        return write!(f, "{}", board_print);
    }
}
