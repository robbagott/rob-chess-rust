use super::chess_move::ChessMove;
use super::color::Color;
use super::game_piece::{GamePiece, Piece};
use std::fmt;

// Square represents a square in a chess position. Squares can have a piece placed on them.
#[derive(Debug)]
pub struct Square {
    pub file: usize,
    pub rank: usize,
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
    pub fn get_moves(&self, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(20);

        /* Only pieces can make moves in chess, so we iterate through the board and check for pieces.
        If a piece is found, we then check for legal moves for that piece. */
        for r in 0..self.board.len() {
            for f in 0..self.board[r].len() {
                if let Some(p) = self.board[r][f] {
                    if p.color == color {
                        moves.append(&mut self.get_moves_at(f, r, color));
                    }
                }
            }
        }

        // Prune moves which lead to checks
        let mut valid_moves = Vec::<ChessMove>::with_capacity(20);
        for m in moves.into_iter() {
            if self.causes_check(&m, color) {
                valid_moves.push(m);
            }
        }

        valid_moves
    }

    fn get_moves_at(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
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

    // TODO
    fn get_rook_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(20);
        Self::add_look_result(&mut moves, color, f, r, self.look_right(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_left(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_up(f, r));
        Self::add_look_result(&mut moves, color, f, r, self.look_down(f, r));
        moves
    }

    // TODO
    fn get_knight_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::with_capacity(8);
        let squares = self.look_l(f, r);
        for s in squares {
            if self.can_move_to_square(f, r, color) {
                moves.push(ChessMove::new(f, r, s.file, s.rank, None));
            }
        }
        moves
    }

    //func (p *Position) getKnightMoves(f, r int, side Side) []Move {
    //    moves := make([]Move, 0, 8)
    //    squares := p.lookL(f, r)
    //    for _, square := range squares {
    //        if canMove, _ := canMoveToSquare(*p, square.file, square.rank, side); canMove {
    //            moves = append(moves, Move{f, r, square.file, square.rank, ""})
    //        }
    //    }
    //    return moves
    //}
    // TODO
    fn get_bishop_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        vec![]
    }
    // TODO
    fn get_queen_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        vec![]
    }
    // TODO
    fn get_king_moves(&self, f: usize, r: usize, color: Color) -> Vec<ChessMove> {
        vec![]
    }

    // TODO
    fn causes_check(&self, m: &ChessMove, color: Color) -> bool {
        false
    }

    /* lookUp and other look functions look in a direction on the board from a starting square.
     * When another piece is encountered, the function returns with the squares traversed and
     * the collision piece. */
    fn look_up(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in (r + 1)..8 {
            squares.push(Square { file: f, rank: i });
            if let None = self.board[i as usize][f] {
                piece = self.board[i as usize][f];
                break;
            }
        }
        (squares, piece)
    }

    //func (p *Position) lookUpRight(f, r int) ([]Square, *GamePiece) {
    //    squares := make([]Square, 0)
    //    piece := GamePiece{None, White}
    //    for i, j := r+1, f+1; i < 8 && j < 8; i, j = i+1, j+1 {
    //        squares = append(squares, Square{j, i})
    //        if p.board[i][j].piece != None {
    //            piece = p.board[i][j]
    //            break
    //        }
    //    }
    //    return squares, &piece
    //}

    fn look_right(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in (f + 1)..8 {
            squares.push(Square { file: i, rank: r });
            if let None = self.board[r][i as usize] {
                piece = self.board[r][i as usize];
                break;
            }
        }
        (squares, piece)
    }

    //func (p *Position) lookDownRight(f, r int) ([]Square, *GamePiece) {
    //    squares := make([]Square, 0)
    //    piece := GamePiece{None, White}
    //    for i, j := r-1, f+1; i >= 0 && j < 8; i, j = i-1, j+1 {
    //        squares = append(squares, Square{j, i})
    //        if p.board[i][j].piece != None {
    //            piece = p.board[i][j]
    //            break
    //        }
    //    }
    //    return squares, &piece
    //}

    fn look_down(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in (r - 1)..=0 {
            squares.push(Square { file: f, rank: i });
            if let None = self.board[i as usize][f] {
                piece = self.board[i as usize][f];
                break;
            }
        }
        (squares, piece)
    }
    //func (p *Position) lookDownLeft(f, r int) ([]Square, *GamePiece) {
    //    squares := make([]Square, 0)
    //    piece := GamePiece{None, White}
    //    for i, j := r-1, f-1; i >= 0 && j >= 0; i, j = i-1, j-1 {
    //        squares = append(squares, Square{j, i})
    //        if p.board[i][j].piece != None {
    //            piece = p.board[i][j]
    //            break
    //        }
    //    }
    //    return squares, &piece
    //}

    fn look_left(&self, f: usize, r: usize) -> (Vec<Square>, Option<GamePiece>) {
        let mut squares = Vec::<Square>::new();
        let mut piece = None;
        for i in (f - 1)..=0 {
            squares.push(Square { file: i, rank: r });
            if let None = self.board[r][i as usize] {
                piece = self.board[r][i as usize];
                break;
            }
        }
        (squares, piece)
    }

    //func (p *Position) lookUpLeft(f, r int) ([]Square, *GamePiece) {
    //    squares := make([]Square, 0)
    //    piece := GamePiece{None, White}
    //    for i, j := r+1, f-1; i < 8 && j >= 0; i, j = i+1, j-1 {
    //        squares = append(squares, Square{j, i})
    //        if p.board[i][j].piece != None {
    //            piece = p.board[i][j]
    //            break
    //        }
    //    }
    //    return squares, &piece
    //}

    //func (p *Position) lookL(f, r int) []Square {
    //    squares := make([]Square, 0)

    //    // Right L moves
    //    if f+2 < 8 && r+1 < 8 {
    //        squares = append(squares, Square{f + 2, r + 1})
    //    }
    //    if f+2 < 8 && r-1 >= 0 {
    //        squares = append(squares, Square{f + 2, r - 1})
    //    }

    //    // Left L moves
    //    if f-2 >= 0 && r+1 < 8 {
    //        squares = append(squares, Square{f - 2, r + 1})
    //    }
    //    if f-2 >= 0 && r-1 >= 0 {
    //        squares = append(squares, Square{f - 2, r - 1})
    //    }

    //    // Forward L moves
    //    if f+1 < 8 && r+2 < 8 {
    //        squares = append(squares, Square{f + 1, r + 2})
    //    }
    //    if f-1 >= 0 && r+2 < 8 {
    //        squares = append(squares, Square{f - 1, r + 2})
    //    }

    //    // Backward L moves
    //    if f+1 < 8 && r-2 >= 0 {
    //        squares = append(squares, Square{f + 1, r - 2})
    //    }
    //    if f-1 >= 0 && r-2 >= 0 {
    //        squares = append(squares, Square{f - 1, r - 2})
    //    }

    //    return squares
    //}

    // TODO
    fn look_l(&self, f: usize, r: usize) -> Vec<Square> {
        let mut squares = Vec::<Square>::new();

        // Right L moves
        if f + 2 < 8 && r + 1 < 8 {
            squares.push(Square {
                file: f + 2,
                rank: r + 1,
            });
        }
        if f + 2 < 8 && r as i32 - 1 >= 0 {
            squares.push(Square {
                file: f + 2,
                rank: r - 1,
            });
        }

        // Left L moves
        if f as i32 - 2 >= 0 && r + 1 < 8 {
            squares.push(Square {
                file: f - 2,
                rank: r + 1,
            });
        }
        if f as i32 - 2 < 8 && r as i32 - 1 >= 0 {
            squares.push(Square {
                file: f - 2,
                rank: r - 1,
            });
        }

        // Forward L moves
        if f + 1 < 8 && r + 2 < 8 {
            squares.push(Square {
                file: f + 1,
                rank: r + 2,
            });
        }
        if f as i32 - 1 >= 0 && r + 2 < 8 {
            squares.push(Square {
                file: f - 1,
                rank: r + 2,
            });
        }

        // Backward L moves
        if f + 1 < 8 && r as i32 - 2 >= 0 {
            squares.push(Square {
                file: f + 1,
                rank: r - 2,
            });
        }
        if f as i32 - 1 >= 0 && r as i32 - 2 >= 0 {
            squares.push(Square {
                file: f - 1,
                rank: r - 2,
            });
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

    // TODO
    fn can_move_to_square(&self, f: usize, r: usize, color: Color) -> bool {
        true
    }
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
