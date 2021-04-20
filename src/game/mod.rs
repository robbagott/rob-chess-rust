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
    let _re = Regex::new("(?P<file1>[a-h])(?P<rank1>[1-8])(?P<file2>[a-h])(?P<rank2>[1-8])(?P<promotionPiece>[bnrq])?").unwrap();
    println!(
        "Welcome to Rob Chess! When entering moves, please use long algebraic chess notation."
    );

    let game_ctx = GameContext::new();
    let side = prompt_color();

    println!("{}", game_ctx.position);

    // game_loop(side, side, game_ctx);
}

fn prompt_color() -> Side {
    println!("Choose a color ('w' or 'b' accepted)");
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);

    match result {
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

fn game_loop(side: Side, player_side: Side, g: GameContext) {
    // var oppSide = side.OppSide()

    // if side == playerSide {
    // 	move := readMove()
    // 	if ok := g.MakeMove(move); !ok {
    // 		fmt.Printf("Something went wrong processing move: %+v\n", move)
    // 		return
    // 	}
    // 	fmt.Println(g.position)
    // 	gameLoop(oppSide, playerSide, g)
    // } else {
    // 	fmt.Printf("I think my moves are %v\n", g.position.GetMoves(side))
    // 	engineMove := Think(g, side)
    // 	fmt.Printf("Engine Move: %v\n", engineMove)
    // 	g.MakeMove(engineMove)
    // 	fmt.Printf("Moves so far: %v\n", g.moves)
    // 	fmt.Println(g.position)
    // 	fmt.Printf("I think your moves are %v\n", g.position.GetMoves(oppSide))
    // 	gameLoop(oppSide, playerSide, g)
    // }
}
