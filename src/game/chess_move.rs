// Move represents a move on the chess board. It encompasses a piece, the old square and the new square.
pub struct ChessMove {
    o_file: i32,
    o_rank: i32,
    n_file: i32,
    n_rank: i32,
    promo_piece: String,
}

impl PartialEq for ChessMove {
    fn eq(&self, other: &Self) -> bool {
        self.o_file == other.o_file
            && self.o_rank == other.o_rank
            && self.n_file == other.n_file
            && self.n_rank == other.n_rank
            && self.promo_piece == other.promo_piece
    }
}
