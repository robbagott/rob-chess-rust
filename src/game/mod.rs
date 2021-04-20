mod chess_move;
mod game_piece;
mod position;
mod side;

use crate::tree::Node;
use crate::tree::Tree;
use chess_move::ChessMove;
use position::Position;
use regex::Regex;
use side::Side;
use std::io;

struct GameContext {
    pub position: Position,
    pub chess_moves: Vec<ChessMove>,
    pub game_tree: Tree<Node<ChessMove>>,
}

impl GameContext {
    fn new() -> GameContext {
        GameContext {
            position: Position::new(),
            chess_moves: Vec::<ChessMove>::new(),
            game_tree: Tree::<Node<ChessMove>>::new(),
        }
    }
}

pub fn start_user_session() {
    println!(
        "Welcome to Rob Chess! When entering moves, please use long algebraic chess notation."
    );

    let game_ctx = GameContext::new();
    let side = prompt_color();

    println!("{}", game_ctx.position);

    game_loop(side, side, game_ctx);
}

fn prompt_color() -> Side {
    println!("Choose a color ('w' or 'b' accepted)");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "w" => return Side::White,
            "b" => return Side::Black,
            _ => {
                println!("Unrecognized input was: {}. Please try again.", input);
                prompt_color()
            }
        },
        Err(err) => {
            println!("Something went wrong reading: {}", err);
            prompt_color()
        }
    }
}

fn get_move() -> ChessMove {
    println!("Move: ");
    let move_str = match read_move() {
        Ok(input) => input,
        Err(err) => {
            println!("Error reading input: {}", err);
            return get_move();
        }
    };

    match algebraic_to_move(&move_str) {
        Ok(res) => res,
        Err(err) => {
            println!("The move entered could not be understood. Please enter a move in long algrebraic chess notation.");
            return get_move();
        }
    }
}

fn read_move() -> Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(x) => Ok(input),
        Err(err) => Err(err),
    }
}

fn algebraic_to_move(alg_move: &str) -> Option<ChessMove> {
    alg_move = &alg_move.to_lowercase().trim();
    let moveEx = Regex::new("(?P<file1>[a-h])(?P<rank1>[1-8])(?P<file2>[a-h])(?P<rank2>[1-8])(?P<promotionPiece>[bnrq])?").unwrap();

    let caps = moveEx.captures(alg_move)?;
    let file1 = caps.get(1)?;
    let rank1 = caps.get(2)?;
    let file2 = caps.get(3)?;
    let rank2 = caps.get(4)?;

    // Optional promotion component
    let promo_piece;
    if caps.len() == 6 {
        promo_piece = caps.get(5)?;
    }

    return Some(ChessMove::new(file1, rank1, file2, rank2, promo_piece));
}

fn game_loop(side: Side, player_side: Side, g: GameContext) {
    let opp_side = side.opp_side();

    if side == player_side {
        let chess_move = get_move();
        //g.make_move(chess_move).expect(format!("Something went wrong processing move: {}\n", chess_move));
        println!("{}", g.position);
        game_loop(opp_side, player_side, g);
    } else {
        // println!("I think my moves are {}\n", g.position.get_moves(side));
        // let engine_move = engine::think(g, side);
        // println!("Engine Move: {}\n", engine_move);
        // g.make_move(engine_move);
        // println!("Moves so far: {}\n", g.moves);
        // println!("{}", g.position);
        // println!("I think your moves are {}}\n", g.position.GetMoves(oppSide));
        // game_loop(opp_side, player_side, g)
    }
}
