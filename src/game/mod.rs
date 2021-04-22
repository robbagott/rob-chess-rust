mod chess_move;
mod color;
mod game_context;
mod game_piece;
mod position;
mod tree;

use chess_move::ChessMove;
use color::Color;
use game_context::GameContext;
use std::io;

pub fn start_user_session() {
    println!(
        "Welcome to Rob Chess! When entering moves, please use long algebraic chess notation."
    );

    let mut game_ctx = GameContext::new();
    let color = prompt_color();

    println!("{}", game_ctx.position);

    game_loop(color, color, &mut game_ctx);
}

fn prompt_color() -> Color {
    println!("Choose a color ('w' or 'b' accepted)");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "w" => return Color::White,
            "b" => return Color::Black,
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
            // Abort assignment and start the process over.
            println!("Error reading input: {}", err);
            return get_move();
        }
    };

    match ChessMove::from_algebraic(&move_str) {
        Some(res) => res,
        None => {
            println!("The move entered could not be understood. Please enter a move in long algrebraic chess notation.");
            return get_move();
        }
    }
}

fn read_move() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map(|_| input)
}

fn game_loop(color: Color, player_color: Color, g: &mut GameContext) {
    let opp_color = color.opp_color();

    if color == player_color {
        let chess_move = get_move();
        println!("{}", chess_move);
        g.make_move(chess_move)
            .expect("Something went wrong processing the move\n");
        println!("{}", g.position);
        game_loop(opp_color, player_color, g);
    } else {
        println!("I think my moves are {:?}\n", g.position.get_moves(color))

        // let engine_move = engine::think(g, color);
        // println!("Engine Move: {}\n", engine_move);
        // g.make_move(engine_move);
        // println!("Moves so far: {}\n", g.moves);
        // println!("{}", g.position);
        // println!("I think your moves are {}}\n", g.position.GetMoves(opp_color));
        // game_loop(opp_color, player_color, g)
    }
}
