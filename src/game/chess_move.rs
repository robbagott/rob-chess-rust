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

        let promo_match = caps.get(5);
        let promo_str = match promo_match {
            Some(p) => Some(p.as_str()), // Optional promotion component
            None => None,
        };
        let promo_piece = match promo_str {
            Some(s) => Some(Piece::from_str(s)),
            None => None,
        };
        let promo_option = match promo_piece {
            Some(Ok(p)) => Some(p),
            _ => None,
        };
        return Some(ChessMove::new(
            file1,
            rank1 as usize,
            file2,
            rank2 as usize,
            promo_option,
        ));
    }

    fn num_to_file(i: usize) -> &'static str {
        match i {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => "x",
        }
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
            Self::num_to_file(self.o_file),
            self.o_rank + 1,
            Self::num_to_file(self.n_file),
            self.n_rank + 1,
            promo_str
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
