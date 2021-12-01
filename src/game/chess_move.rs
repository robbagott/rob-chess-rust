use super::game_piece::Piece;
use regex::Regex;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

const ALGEBRAIC_REGEX: &str =
    "(?P<file1>[a-h])(?P<rank1>[1-8])(?P<file2>[a-h])(?P<rank2>[1-8])(?P<promotionPiece>[bnrq])?";

// Move represents a move on the chess board. It encompasses a piece, the old square and the new square.
#[derive(Debug, Copy, Clone)]
pub struct ChessMove {
    pub o_file: usize,
    pub o_rank: usize,
    pub n_file: usize,
    pub n_rank: usize,
    pub promo_piece: Option<Piece>,
}

impl ChessMove {
    pub fn new(
        o_file: usize,
        o_rank: usize,
        n_file: usize,
        n_rank: usize,
        promo_piece: Option<Piece>,
    ) -> ChessMove {
        ChessMove {
            o_file,
            o_rank,
            n_file,
            n_rank,
            promo_piece,
        }
    }
    pub fn from_algebraic(alg_move: &str) -> Option<ChessMove> {
        let sanitized = &alg_move.trim().to_lowercase();
        let move_ex = Regex::new(ALGEBRAIC_REGEX).unwrap();
        let caps = move_ex.captures(sanitized)?;
        let file1 = parse_file(caps.get(1)?.as_str())?;
        let rank1 = parse_rank(caps.get(2)?.as_str())?;
        let file2 = parse_file(caps.get(3)?.as_str())?;
        let rank2 = parse_rank(caps.get(4)?.as_str())?;
        let promo_str = caps.get(5).map(|x| String::from(x.as_str())); // Optional promotion component
        let promo_piece = Piece::from_str(&promo_str.unwrap());
        let promo_piece_option = match promo_piece {
            Ok(piece) => Some(piece),
            Err(_) => None,
        };
        return Some(ChessMove::new(
            file1,
            rank1 as usize,
            file2,
            rank2 as usize,
            promo_piece_option,
        ));
    }
}

impl PartialEq for ChessMove {
    fn eq(&self, other: &Self) -> bool {
        let equal = self.o_file == other.o_file
            && self.o_rank == other.o_rank
            && self.n_file == other.n_file
            && self.n_rank == other.n_rank;

        let promo_equal = match (&self.promo_piece, &other.promo_piece) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        };

        return equal && promo_equal;
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let promo_str = match self.promo_piece {
            Some(p) => p.to_string(),
            None => "".to_owned(),
        };
        write!(
            f,
            "{}{}{}{}{}",
            self.o_file, self.o_rank, self.n_file, self.n_rank, promo_str
        )
    }
}

fn parse_file(file: &str) -> Option<usize> {
    let num_char = file.chars().next()?.to_lowercase().next()?;
    let digit = (num_char as usize) - ('a' as usize);
    Some(digit)
}

fn parse_rank(rank: &str) -> Option<usize> {
    rank.parse::<usize>().ok().map(|n| n - 1)
}
