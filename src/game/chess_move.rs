use regex::Regex;
use std::fmt;
use std::fmt::Display;

const ALGEBRAIC_REGEX: &str =
    "(?P<file1>[a-h])(?P<rank1>[1-8])(?P<file2>[a-h])(?P<rank2>[1-8])(?P<promotionPiece>[bnrq])?";

// Move represents a move on the chess board. It encompasses a piece, the old square and the new square.
#[derive(Debug)]
pub struct ChessMove {
    o_file: u32,
    o_rank: u32,
    n_file: u32,
    n_rank: u32,
    promo_piece: Option<String>,
}

impl ChessMove {
    pub fn new(
        o_file: u32,
        o_rank: u32,
        n_file: u32,
        n_rank: u32,
        promo_piece: Option<String>,
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
        let promo_piece = caps.get(5).map(|x| String::from(x.as_str())); // Optional promotion component
        return Some(ChessMove::new(
            file1,
            rank1 as u32,
            file2,
            rank2 as u32,
            promo_piece,
        ));
    }
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

impl Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let promo_str = self.promo_piece.as_deref().unwrap_or("");
        write!(
            f,
            "{}{}{}{}{}",
            self.o_file, self.o_rank, self.n_file, self.n_rank, promo_str
        )
    }
}

fn parse_file(file: &str) -> Option<u32> {
    let num_char = file.chars().next()?.to_lowercase().next()?;
    let digit = (num_char as u32) - ('a' as u32);
    Some(digit)
}

fn parse_rank(rank: &str) -> Option<u32> {
    rank.parse::<u32>().ok().map(|n| n - 1)
}
