// White and Black are the values for a piece color.
#[derive(Copy, Clone, PartialEq)]
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
