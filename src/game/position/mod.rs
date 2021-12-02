#[cfg(test)]
mod position_tests;

use super::chess_move::ChessMove;
use super::color::Color;
use super::game_piece::{GamePiece, Piece};
use std::fmt;
use std::iter::Rev;
use std::ops::Range;

// Square represents a square in a chess position. Squares can have a piece placed on them.
#[derive(Debug, PartialEq)]
pub struct Square {
    pub file: usize,
    pub rank: usize,
}

impl Square {
    pub fn new(f: usize, r: usize) -> Square {
        Square { file: f, rank: r }
    }
}

type Board = [[Option<GamePiece>; 8]; 8];

// Position represents a chess position representation.
#[derive(Debug)]
pub struct Position {
    pub board: Board,
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

        self.o_o_o_white = true;
        self.o_o_white = true;
        self.o_o_o_black = true;
        self.o_o_black = true;
    }

    pub fn make_move(&mut self, chess_move: &ChessMove) -> Result<(), ()> {
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
    // TODO fix self mutability.
    pub fn get_moves(&mut self, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(20);

        /* Only pieces can make moves in chess, so we iterate through the board and check for pieces.
        If a piece is found, we then check for legal moves for that piece. */
        for r in 0..self.board.len() {
            for f in 0..self.board[r].len() {
                if let Some(p) = self.board[r][f] {
                    if p.color == color {
                        moves.append(&mut self.get_moves_at(f, r));
                    }
                }
            }
        }

        // Prune moves which lead to checks
        let mut valid_moves = Vec::<ChessMove>::with_capacity(20);
        for m in moves.into_iter() {
            //            if self.causes_check(&m, color) {
            valid_moves.push(m);
            //            }
        }

        valid_moves
    }

    fn get_moves_at(&self, f: usize, r: usize) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(20);
        let piece = self.board[r][f];
        if piece.is_none() {
            return vec![];
        }
        let piece = piece.unwrap();
        match piece.piece {
            Piece::Pawn => moves.append(&mut self.get_pawn_moves(f, r, piece.color)),
            Piece::Rook => moves.append(&mut self.get_rook_moves(f, r, piece.color)),
            Piece::Knight => moves.append(&mut self.get_knight_moves(f, r, piece.color)),
            Piece::Bishop => moves.append(&mut self.get_bishop_moves(f, r, piece.color)),
            Piece::Queen => moves.append(&mut self.get_queen_moves(f, r, piece.color)),
            Piece::King => moves.append(&mut self.get_king_moves(f, r, piece.color)),
        };

        moves
    }

    // TODO teach about en passant.
    fn get_pawn_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(4);

        let r_incr: i32 = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        // TODO remove and teach about promotion.
        if r == 7 && color == Color::White || r == 0 && color == Color::Black {
            return moves;
        }

        // Possible forward moves
        let r_up = (r as i32 + r_incr) as usize;
        let r_up_2 = (r as i32 + r_incr * 2) as usize;
        if r == 1 && color == Color::White || r == 6 && color == Color::Black {
            if let None = self.board[r_up][f] {
                moves.push(ChessMove::new(f, r, f, r_up, None));
                if let None = self.board[r_up_2][f] {
                    moves.push(ChessMove::new(f, r, f, r_up_2, None));
                }
            }
        } else if r > 1 && color == Color::White || r < 6 && color == Color::Black {
            if let None = self.board[r_up][f] {
                moves.push(ChessMove::new(f, r, f, r_up, None))
            }
        }

        // Possible captures
        if f as i32 - 1 >= 0 {
            let f_left = (f as i32 - 1) as usize;
            let capture_piece = self.board[r_up][f_left];
            match capture_piece {
                Some(p) if p.color != color => moves.push(ChessMove::new(f, r, f_left, r_up, None)),
                _ => (),
            }
        }

        let f_right = f + 1;
        if f_right <= 7 {
            let capture_position = self.board[r_up][f_right];
            match capture_position {
                Some(p) if p.color != color => {
                    moves.push(ChessMove::new(f, r, f_right, r_up, None))
                }
                _ => (),
            }
        }
        moves
    }

    fn get_rook_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(20);
        Self::add_look_result(&mut moves, color, f, r, self.look_right(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_left(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_up(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_down(f, r));
        moves
    }

    fn get_knight_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(8);
        let squares = self.look_l(f, r);
        for s in squares {
            if self.can_move_to_square(s.file as isize, s.rank as isize, color) {
                moves.push(ChessMove::new(f, r, s.file, s.rank, None));
            }
        }
        moves
    }

    fn get_bishop_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(20);

        Self::add_look_result(&mut moves, color, f, r, self.look_up_right(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_up_left(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_down_right(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_down_left(f, r));
        moves
    }

    fn get_queen_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = self.get_rook_moves(f, r, color);
        moves.extend(self.get_bishop_moves(f, r, color));
        moves
    }

    fn get_king_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(8);

        let fi = f as isize;
        let ri = r as isize;

        // Right
        if self.can_move_to_square(fi + 1, ri, color) {
            moves.push(ChessMove::new(f, r, f + 1, r, None));
        }
        // Down right
        if self.can_move_to_square(fi + 1, ri - 1, color) {
            moves.push(ChessMove::new(f, r, f + 1, r - 1, None));
        }
        // Down
        if self.can_move_to_square(fi, ri - 1, color) {
            moves.push(ChessMove::new(f, r, f, r - 1, None));
        }
        // Down left
        if self.can_move_to_square(fi - 1, ri - 1, color) {
            moves.push(ChessMove::new(f, r, f - 1, r - 1, None));
        }
        // Left
        if self.can_move_to_square(fi - 1, ri, color) {
            moves.push(ChessMove::new(f, r, f - 1, r, None));
        }
        // Up left
        if self.can_move_to_square(fi - 1, ri + 1, color) {
            moves.push(ChessMove::new(f, r, f - 1, r + 1, None));
        }
        // Up
        if self.can_move_to_square(fi, ri + 1, color) {
            moves.push(ChessMove::new(f, r, f, r + 1, None));
        }
        // Up right
        if self.can_move_to_square(fi + 1, ri + 1, color) {
            moves.push(ChessMove::new(f, r, f + 1, r + 1, None));
        }

        // TODO Castle
        moves
    }

    // TODO self should probably not be mutable in this function. Fix later.
    fn causes_check(&mut self, m: &ChessMove, color: Color) -> bool {
        let old_piece = self.board[m.o_rank][m.o_file];
        let captured_piece = self.board[m.n_rank][m.n_file];
        let mut to_return = false;

        self.make_move(&m)
            .expect("Could not make expected move in causes_check().");
        let king_square = self.get_king_square(color);
        if self.in_check(king_square.file, king_square.rank, color) {
            to_return = true;
        }

        // Roll back the move.
        self.board[m.o_rank][m.o_file] = old_piece;
        self.board[m.n_rank][m.n_file] = captured_piece;
        to_return
    }

    fn get_king_square(&self, color: Color) -> Square {
        for r in 0..self.board.len() {
            for f in 0..self.board[r].len() {
                if let Some(p) = self.board[r][f] {
                    if p.color == color {
                        return Square::new(f, r);
                    }
                }
            }
        }
        panic!("No king found in position");
    }

    // TODO Expand to king and pawns.
    fn in_check(&self, f: usize, r: usize, color: Color) -> bool {
        let check_straight = |func: fn(&Self, usize, usize) -> (Vec<Square>, Option<GamePiece>)| {
            if let (_, Some(p)) = func(self, f, r) {
                match p.piece {
                    Piece::Rook | Piece::Queen if p.color == color => return true,
                    _ => return false,
                }
            }
            false
        };
        let check_diagonal = |func: fn(&Self, usize, usize) -> (Vec<Square>, Option<GamePiece>)| {
            if let (_, Some(p)) = func(self, f, r) {
                match p.piece {
                    Piece::Bishop | Piece::Queen if p.color == color => return true,
                    _ => return false,
                }
            }
            false
        };
        check_straight(Self::look_up)
            || check_straight(Self::look_right)
            || check_straight(Self::look_down)
            || check_straight(Self::look_left)
            || check_diagonal(Self::look_up_right)
            || check_diagonal(Self::look_down_right)
            || check_diagonal(Self::look_down_left)
            || check_diagonal(Self::look_up_right)
    }

    /* lookUp and other look functions look in a direction on the board from a starting square.
     * When another piece is encountered, the function returns with the squares traversed and
     * the collision piece. */
    fn look_up(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in BoardRange::new(r, 1, 7) {
            squares.push(Square::new(f, i));
            if self.board[i as usize][f].is_some() {
                piece = self.board[i as usize][f];
                break;
            }
        }
        (squares, piece)
    }

    fn look_up_right(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::with_capacity(20);
        let mut piece = None;
        for i in BoardRange::new(r, 1, 7) {
            let j = f + (i - r);
            if j > 7 {
                break;
            }
            squares.push(Square::new(j, i));
            if self.board[i][j].is_some() {
                piece = self.board[i][j];
                break;
            }
        }
        (squares, piece)
    }

    fn look_right(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in BoardRange::new(f, 1, 7) {
            squares.push(Square::new(i, r));
            if self.board[r][i as usize].is_some() {
                piece = self.board[r][i as usize];
                break;
            }
        }
        (squares, piece)
    }

    fn look_down_right(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::with_capacity(20);
        let mut piece = None;
        for i in BoardRange::new(r, -1, 0) {
            let j = f + (r - i);
            if j > 7 {
                break;
            }
            squares.push(Square::new(j, i));
            if self.board[i][j].is_some() {
                piece = self.board[i][j];
                break;
            }
        }
        (squares, piece)
    }

    fn look_down(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in BoardRange::new(r, -1, 0) {
            squares.push(Square::new(f, i));
            if self.board[i as usize][f].is_some() {
                piece = self.board[i as usize][f];
                break;
            }
        }
        (squares, piece)
    }

    fn look_down_left(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::with_capacity(20);
        let mut piece = None;
        for i in BoardRange::new(r, -1, 0) {
            let j = f as isize - (r as isize - i as isize);
            if j < 0 {
                break;
            }
            squares.push(Square::new(j as usize, i));
            if self.board[i][j as usize].is_some() {
                piece = self.board[i][j as usize];
                break;
            }
        }
        (squares, piece)
    }

    fn look_left(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in BoardRange::new(f, -1, 0) {
            squares.push(Square::new(i as usize, r));
            if self.board[r][i as usize].is_some() {
                piece = self.board[r][i as usize];
                break;
            }
        }
        (squares, piece)
    }

    fn look_up_left(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::with_capacity(20);
        let mut piece = None;
        for i in BoardRange::new(r, 1, 7) {
            let j = f as isize - (i as isize - r as isize);
            if j < 0 {
                break;
            }
            squares.push(Square::new(j as usize, i));
            if self.board[i][j as usize].is_some() {
                piece = self.board[i][j as usize];
                break;
            }
        }
        (squares, piece)
    }

    fn look_l(&self, f: usize, r: usize) -> Vec<Square> {
        let mut squares = Vec::<Square>::new();

        // Right L moves
        if f + 2 < 8 && r + 1 < 8 {
            squares.push(Square::new(f + 2, r + 1));
        }
        if f + 2 < 8 && r as i32 - 1 >= 0 {
            squares.push(Square::new(f + 2, r - 1));
        }

        // Left L moves
        if f as i32 - 2 >= 0 && r + 1 < 8 {
            squares.push(Square::new(f - 2, r + 1));
        }
        if f as i32 - 2 >= 0 && r as i32 - 1 >= 0 {
            squares.push(Square::new(f - 2, r - 1));
        }

        // Forward L moves
        if f + 1 < 8 && r + 2 < 8 {
            squares.push(Square::new(f + 1, r + 2));
        }
        if f as i32 - 1 >= 0 && r + 2 < 8 {
            squares.push(Square::new(f - 1, r + 2));
        }

        // Backward L moves
        if f + 1 < 8 && r as i32 - 2 >= 0 {
            squares.push(Square::new(f + 1, r - 2));
        }
        if f as i32 - 1 >= 0 && r as i32 - 2 >= 0 {
            squares.push(Square::new(f - 1, r - 2));
        }
        squares
    }

    fn add_look_result(
        moves: &mut Vec<ChessMove>,
        color: Color,
        f: usize,
        r: usize,
        look_result: (Vec<Square>, Option<GamePiece>),
    ) -> () {
        let (squares, piece) = look_result;
        for (i, s) in squares.iter().enumerate() {
            if i == squares.len() - 1 && piece.is_some() && piece.unwrap().color == color {
                continue;
            }
            moves.push(ChessMove::new(f, r, s.file, s.rank, None));
        }
    }

    // Returns if a piece can move to a specific square.
    fn can_move_to_square(&self, f: isize, r: isize, color: Color) -> bool {
        if f < 0 || f > 7 || r < 0 || r > 7 {
            return false;
        }

        match self.board[r as usize][f as usize] {
            Some(p) => {
                if p.color == color {
                    false
                } else {
                    true
                }
            }
            None => true,
        }
    }
}

// Type for generating a range for iterating the board in a safe and convenient way.
enum BoardRange {
    Forward(Range<usize>),
    Backward(Rev<Range<usize>>),
}

impl Iterator for BoardRange {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self {
            BoardRange::Forward(range) => range.next(),
            BoardRange::Backward(range) => range.next(),
        }
    }
}

// Note that a range starting with a negative start or end will cause an empty range to be created.
// Note that these ranges are inclusive at both bounds.
impl BoardRange {
    fn new(start: usize, start_modifier: isize, end: usize) -> BoardRange {
        let start = start as isize + start_modifier;
        let end = end as isize;
        // BoardRanges that begin off the board should be empty.
        if (start > 7 || start < 0) {
            return BoardRange::Forward(0..0);
        }
        if start >= end {
            return BoardRange::Backward((end as usize..(start + 1) as usize).rev());
        }
        BoardRange::Forward(start as usize..(end + 1) as usize)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_print = String::new();
        board_print += "   ––––––––––––––––-----------------\n";
        for r in (0..self.board.len()).rev() {
            board_print += &format!(" {} ", (r as i32 + 1));
            for f in 0..self.board[r].len() {
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
