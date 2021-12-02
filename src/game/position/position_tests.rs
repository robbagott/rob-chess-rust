use crate::game::color::Color;
use crate::game::game_piece::GamePiece;
use crate::game::game_piece::Piece;
use crate::game::position::{BoardRange, Position, Square};

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
fn look_up_right_returns_expected_squares() {
    let mut p = Position::new();
    p.board[6][6] = Some(GamePiece::new(Piece::Pawn, Color::Black));

    let squares = p.look_up_right(2, 2);
    assert_eq!(squares.0.len(), 4);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );

    assert_eq!(squares.0[0], Square::new(3, 3));
    assert_eq!(squares.0[1], Square::new(4, 4));
    assert_eq!(squares.0[2], Square::new(5, 5));
    assert_eq!(squares.0[3], Square::new(6, 6));
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
fn look_down_right_returns_expected_squares() {
    let mut p = Position::new();
    p.board[2][6] = Some(GamePiece::new(Piece::Pawn, Color::Black));

    let squares = p.look_down_right(2, 6);
    println!("{:?}", squares);
    assert_eq!(squares.0.len(), 4);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );

    assert_eq!(squares.0[0], Square::new(3, 5));
    assert_eq!(squares.0[1], Square::new(4, 4));
    assert_eq!(squares.0[2], Square::new(5, 3));
    assert_eq!(squares.0[3], Square::new(6, 2));
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
fn look_down_left_returns_expected_squares() {
    let mut p = Position::new();
    p.board[2][2] = Some(GamePiece::new(Piece::Pawn, Color::Black));

    let squares = p.look_down_left(6, 6);
    assert_eq!(squares.0.len(), 4);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );

    assert_eq!(squares.0[0], Square::new(5, 5));
    assert_eq!(squares.0[1], Square::new(4, 4));
    assert_eq!(squares.0[2], Square::new(3, 3));
    assert_eq!(squares.0[3], Square::new(2, 2));
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

#[test]
fn look_up_left_returns_expected_squares() {
    let p = Position::new();

    let squares = p.look_up_left(3, 3);
    assert_eq!(squares.0.len(), 3);
    assert_eq!(
        squares.1.unwrap(),
        GamePiece::new(Piece::Pawn, Color::Black)
    );

    assert_eq!(squares.0[0], Square::new(2, 4));
    assert_eq!(squares.0[1], Square::new(1, 5));
    assert_eq!(squares.0[2], Square::new(0, 6));
}

#[test]
fn look_l_returns_expected_squares() {
    let p = Position::new();

    let squares = p.look_l(4, 3);

    println!("{:?}", squares);

    assert_eq!(squares[0], Square::new(6, 4));
    assert_eq!(squares[1], Square::new(6, 2));
    assert_eq!(squares[2], Square::new(2, 4));
    assert_eq!(squares[3], Square::new(2, 2));
    assert_eq!(squares[4], Square::new(5, 5));
    assert_eq!(squares[5], Square::new(3, 5));
    assert_eq!(squares[6], Square::new(5, 1));
    assert_eq!(squares[7], Square::new(3, 1));
}

#[test]
fn can_move_to_square_works() {
    let p = Position::new();

    assert_eq!(p.can_move_to_square(0, 0, Color::Black), true);
    assert_eq!(p.can_move_to_square(0, 0, Color::White), false);
    assert_eq!(p.can_move_to_square(3, 4, Color::White), true);
    assert_eq!(p.can_move_to_square(3, 4, Color::Black), true);
    assert_eq!(p.can_move_to_square(8, 4, Color::Black), false);
    assert_eq!(p.can_move_to_square(2, 5, Color::Black), true);
}

#[test]
fn board_range_forward_works() {
    let mut r = BoardRange::new(0, 0, 7);
    match r {
        BoardRange::Forward(ref range) => {
            assert_eq!(range.len(), 8);
        }
        BoardRange::Backward(_) => panic!("Board range is unexpectedly backward."),
    }

    assert_eq!(r.next(), Some(0));
    assert_eq!(r.next(), Some(1));
    assert_eq!(r.next(), Some(2));
    assert_eq!(r.next(), Some(3));
    assert_eq!(r.next(), Some(4));
    assert_eq!(r.next(), Some(5));
    assert_eq!(r.next(), Some(6));
    assert_eq!(r.next(), Some(7));
    assert_eq!(r.next(), None);
}

#[test]
fn board_range_forward_with_mod_works() {
    let mut r = BoardRange::new(0, 1, 7);
    match r {
        BoardRange::Forward(ref range) => {
            assert_eq!(range.len(), 7);
        }
        BoardRange::Backward(_) => panic!("Board range is unexpectedly backward."),
    }

    assert_eq!(r.next(), Some(1));
    assert_eq!(r.next(), Some(2));
    assert_eq!(r.next(), Some(3));
    assert_eq!(r.next(), Some(4));
    assert_eq!(r.next(), Some(5));
    assert_eq!(r.next(), Some(6));
    assert_eq!(r.next(), Some(7));
    assert_eq!(r.next(), None);
}

#[test]
fn board_range_backward_works() {
    let mut r = BoardRange::new(7, 0, 0);
    match r {
        BoardRange::Backward(ref range) => {
            assert_eq!(range.len(), 8);
        }
        BoardRange::Forward(_) => panic!("Board range is unexpectedly forward."),
    }

    assert_eq!(r.next(), Some(7));
    assert_eq!(r.next(), Some(6));
    assert_eq!(r.next(), Some(5));
    assert_eq!(r.next(), Some(4));
    assert_eq!(r.next(), Some(3));
    assert_eq!(r.next(), Some(2));
    assert_eq!(r.next(), Some(1));
    assert_eq!(r.next(), Some(0));
    assert_eq!(r.next(), None);
}

#[test]
fn board_range_backward_with_mod_works() {
    let mut r = BoardRange::new(7, -1, 0);
    match r {
        BoardRange::Backward(ref range) => {
            assert_eq!(range.len(), 7);
        }
        BoardRange::Forward(_) => panic!("Board range is unexpectedly forward."),
    }

    assert_eq!(r.next(), Some(6));
    assert_eq!(r.next(), Some(5));
    assert_eq!(r.next(), Some(4));
    assert_eq!(r.next(), Some(3));
    assert_eq!(r.next(), Some(2));
    assert_eq!(r.next(), Some(1));
    assert_eq!(r.next(), Some(0));
    assert_eq!(r.next(), None);
}

#[test]
fn board_range_start_oob_empty() {
    let r = BoardRange::new(8, 7, 7);

    match r {
        BoardRange::Forward(ref range) => {
            assert_eq!(range.len(), 0);
        }
        BoardRange::Backward(_) => panic!("Board range is unexpectedly backward."),
    }

    let r1 = BoardRange::new(7, 1, 7);

    match r1 {
        BoardRange::Forward(ref range) => {
            assert_eq!(range.len(), 0);
        }
        BoardRange::Backward(_) => panic!("Board range is unexpectedly backward."),
    }
}
