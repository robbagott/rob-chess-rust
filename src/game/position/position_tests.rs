use crate::game::color::Color;
use crate::game::game_piece::GamePiece;
use crate::game::game_piece::Piece;
use crate::game::position::Position;

#[test]
fn reset_resets_castling_rights() {
    let mut p = Position::new();
    p.o_o_o_white = false;
    p.o_o_white = false;
    p.o_o_o_black = false;
    p.o_o_black = false;

    p.reset();

    assert_eq!(p.o_o_o_white, true);
    assert_eq!(p.o_o_white, true);
    assert_eq!(p.o_o_o_black, true);
    assert_eq!(p.o_o_black, true);
}

#[test]
fn reset_places_pieces_correctly() {
    let mut p = Position::new();
    p.reset();

    assert_eq!(
        p.board[0][0].unwrap(),
        GamePiece::new(Piece::Rook, Color::White)
    );
    assert_eq!(
        p.board[0][1].unwrap(),
        GamePiece::new(Piece::Knight, Color::White)
    );
    assert_eq!(
        p.board[0][2].unwrap(),
        GamePiece::new(Piece::Bishop, Color::White)
    );
    assert_eq!(
        p.board[0][3].unwrap(),
        GamePiece::new(Piece::Queen, Color::White)
    );
    assert_eq!(
        p.board[0][4].unwrap(),
        GamePiece::new(Piece::King, Color::White)
    );
    assert_eq!(
        p.board[0][5].unwrap(),
        GamePiece::new(Piece::Bishop, Color::White)
    );
    assert_eq!(
        p.board[0][6].unwrap(),
        GamePiece::new(Piece::Knight, Color::White)
    );
    assert_eq!(
        p.board[0][7].unwrap(),
        GamePiece::new(Piece::Rook, Color::White)
    );
    assert_eq!(
        p.board[1][0].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
    assert_eq!(
        p.board[1][1].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
    assert_eq!(
        p.board[1][2].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
    assert_eq!(
        p.board[1][3].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
    assert_eq!(
        p.board[1][4].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
    assert_eq!(
        p.board[1][5].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
    assert_eq!(
        p.board[1][6].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
    assert_eq!(
        p.board[1][7].unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );

    assert_eq!(
        p.board[7][0].unwrap(),
        GamePiece::new(Piece::Rook, Color::Black)
    );
    assert_eq!(
        p.board[7][1].unwrap(),
        GamePiece::new(Piece::Knight, Color::Black)
    );
    assert_eq!(
        p.board[7][2].unwrap(),
        GamePiece::new(Piece::Bishop, Color::Black)
    );
    assert_eq!(
        p.board[7][3].unwrap(),
        GamePiece::new(Piece::Queen, Color::Black)
    );
    assert_eq!(
        p.board[7][4].unwrap(),
        GamePiece::new(Piece::King, Color::Black)
    );
    assert_eq!(
        p.board[7][5].unwrap(),
        GamePiece::new(Piece::Bishop, Color::Black)
    );
    assert_eq!(
        p.board[7][6].unwrap(),
        GamePiece::new(Piece::Knight, Color::Black)
    );
    assert_eq!(
        p.board[7][7].unwrap(),
        GamePiece::new(Piece::Rook, Color::Black)
    );
    assert_eq!(
        p.board[6][0].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
    assert_eq!(
        p.board[6][1].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
    assert_eq!(
        p.board[6][2].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
    assert_eq!(
        p.board[6][3].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
    assert_eq!(
        p.board[6][4].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
    assert_eq!(
        p.board[6][5].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
    assert_eq!(
        p.board[6][6].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
    assert_eq!(
        p.board[6][7].unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
}

#[test]
fn look_up_returns_expected_squares() {
    let mut p = Position::new();
    p.board[1][0] = None;
    p.board[6][0] = Some(GamePiece::new(Piece::Pawn, Color::Black));

    let squares = p.look_up(0, 0);
    assert_eq!(squares.0.len(), 6);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
}

#[test]
fn look_right_returns_expected_squares() {
    let mut p = Position::new();
    p.board[2][5] = Some(GamePiece::new(Piece::Pawn, Color::Black));

    let squares = p.look_right(0, 2);
    assert_eq!(squares.0.len(), 5);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
}

#[test]
fn look_down_returns_expected_squares() {
    let mut p = Position::new();
    p.board[6][0] = None;

    let squares = p.look_down(0, 7);
    assert_eq!(squares.0.len(), 6);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::White)
    );
}

#[test]
fn look_left_returns_expected_squares() {
    let mut p = Position::new();
    p.board[2][2] = Some(GamePiece::new(Piece::Pawn, Color::Black));

    let squares = p.look_left(7, 2);
    assert_eq!(squares.0.len(), 5);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );
}
