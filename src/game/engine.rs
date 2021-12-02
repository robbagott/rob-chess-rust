use super::chess_move::ChessMove;
use super::color::Color;
use super::game_context::GameContext;
use super::position::Position;
use super::tree::Node;
use std::cmp::Ordering;

static THINK_DEPTH: u32 = 4;

pub fn think(g: &mut GameContext, color: Color) -> ChessMove {
    // Get an initial move
    think_depth(g, color, THINK_DEPTH)
}

// TODO
pub fn think_depth(g: &mut GameContext, color: Color, depth: u32) -> ChessMove {
    let p = &mut g.position;

    // Check if there are moves on the node. If not, retrieve them and add them to the node.
    if g.tree.children.len() == 0 {
        let moves = p.get_moves(color);
        for chess_move in moves {
            g.tree.children.push(Node::new(chess_move, None));
        }
    }

    // TODO remove debugging...
    let start_moves = &mut vec![];
    for child in &g.tree.children {
        start_moves.push(&child.chess_move);
    }
    println!(
        "think_depth start: I think my moves are {}",
        start_moves
            .iter()
            .fold(String::new(), |acc, &arg| acc + ", " + &arg.to_string())
    );

    // Calculate possible moves
    let mut alpha = f64::INFINITY;
    let beta = f64::NEG_INFINITY;
    let mut best_so_far = f64::NEG_INFINITY;
    let mut best_move_so_far = None;
    for child in g.tree.children.iter_mut() {
        let curr_move = &child.chess_move;
        let or = curr_move.o_rank;
        let of = curr_move.o_file;
        let nr = curr_move.n_rank;
        let nf = curr_move.n_file;

        // Keep track of move details so we can roll back.
        let old_piece = p.board[curr_move.o_rank][curr_move.o_file];
        let captured_piece = p.board[curr_move.n_rank][curr_move.n_file];

        match p.make_move(&curr_move) {
            Err(_) => panic!("Unable to make move! in think_depth!"),
            _ => (),
        };

        let eval = -calculate(p, color.opp_color(), depth, -beta, -alpha, child);
        if eval > best_so_far {
            best_move_so_far = Some(child.chess_move);
            best_so_far = eval
        }

        alpha = f64::max(alpha, best_so_far);

        p.board[or][of] = old_piece;
        p.board[nr][nf] = captured_piece;
    }

    match best_move_so_far {
        Some(m) => m,
        None => panic!(),
    }
}

// Calculate is an implementation of negaMax. Perhaps someday it will implement negaScout.
/* Alpha is like a higher order bestSoFar variable. For the maximizer, it is the minimum score we are assured in other branches that we have calculated in parent nodes.
Therefore, if the minimizer in the current branch assures a worse score for us with any of its replies, we can give up on the current branch altogether as the maximizer.
This logic is somewhat muddied by the negamax take on minimax. Alpha typically tracks the maximizer's assured score and beta typically tracks the
minimizer's assured score. In negaMax, we negate the minimizer's result in the call to Calculate() which allows us to share the calculate function
between the two. In order for alpha and beta to work, their values must match with whether the minimizer or the maximizer is evaluating. Now, when
we pass from the maximizer to the minimizer, we give the minimizer beta as its alpha and vice versa. */
fn calculate(
    p: &mut Position,
    color: Color,
    depth: u32,
    alpha: f64,
    beta: f64,
    node: &mut Node,
) -> f64 {
    if depth == 0 {
        return evaluate(&p, color);
    }

    // Check if there are no moves on the node. If not, retrieve them and add them to the node.
    if node.children.len() == 0 {
        let moves = p.get_moves(color);
        for chess_move in moves {
            node.children.push(Node::new(chess_move, None));
        }
    }

    // Calculate possible moves
    let mut best_so_far = f64::NEG_INFINITY;
    let mut alpha = alpha;
    for child in node.children.iter_mut() {
        let chess_move = child.chess_move;
        let old_piece = p.board[chess_move.o_rank][chess_move.o_file];
        let captured_piece = p.board[chess_move.n_rank][chess_move.n_file];
        match p.make_move(&chess_move) {
            Ok(_) => (),
            Err(_) => println!("Failed to make move {}", chess_move),
        };
        child.eval = Some(-calculate(
            p,
            color.opp_color(),
            depth - 1,
            -beta,
            -alpha,
            child,
        ));
        best_so_far = f64::max(best_so_far, child.eval.unwrap());

        // Roll back move.
        p.board[chess_move.o_rank][chess_move.o_file] = old_piece;
        p.board[chess_move.n_rank][chess_move.n_file] = captured_piece;

        alpha = f64::max(alpha, best_so_far);
        if alpha > -beta {
            sort_moves(&mut node.children);
            return best_so_far;
        }

        // From possible moves, choose optimal move. Return the optimal move with its evaluation.
        sort_moves(&mut node.children);
        return best_so_far;
    }
    best_so_far
}

// TODO
fn evaluate(_p: &Position, _color: Color) -> f64 {
    return 0.0;
}

// TODO Fix unsafe unwrap in elegant (hopefully) way.
fn sort_moves(moves: &mut Vec<Node>) -> () {
    moves.sort_by(|a, b| match (a.eval, b.eval) {
        (Some(a), Some(b)) => b.partial_cmp(&a).unwrap(),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        _ => Ordering::Equal,
    });
}
