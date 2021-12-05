use super::chess_move::ChessMove;
use super::color::Color;
use super::game_context::GameContext;
use super::position::Position;
use super::tree::Node;
use std::cmp::Ordering;

static THINK_DEPTH: u32 = 6;

pub fn think(g: &mut GameContext, color: Color) -> ChessMove {
    // Get an initial move
    let mut best_move = None;
    for i in 2..THINK_DEPTH {
        println!("Thinking to depth {}", i);
        best_move = Some(think_depth(g, color, i));
    }
    best_move.expect("Think failed!")
}

// TODO
pub fn think_depth(g: &mut GameContext, color: Color, depth: u32) -> ChessMove {
    let p = &mut g.position;

    println!("{:?} color", color);
    // Check if there are moves on the node. If not, retrieve them and add them to the node.
    if g.tree.children.len() == 0 {
        println!("children not found");
        let moves = p.get_moves(color);
        for chess_move in moves {
            g.tree.children.push(Node::new(Some(chess_move), None));
        }
    }

    // TODO remove debugging...
    //let mut start_moves: Vec<&Option<ChessMove>> = vec![];
    //for child in &g.tree.children {
    //    start_moves.push(&child.last_move);
    //}
    //println!(
    //    "think_depth start: I think my moves are {}",
    //    start_moves.into_iter().fold(String::new(), |acc, arg| acc
    //        + ", "
    //        + &arg.as_ref().map(|m| m.to_string()).unwrap_or("".to_owned()))
    //);

    // Calculate possible moves
    let (mut eval, best_move) = calculate(
        p,
        color,
        depth,
        f64::NEG_INFINITY,
        f64::INFINITY,
        &mut g.tree,
    );
    eval = -eval;
    println!("eval = {}", eval);
    best_move.expect("Did not receive move from calculate")
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
    mut alpha: f64,
    beta: f64,
    node: &mut Node,
) -> (f64, Option<ChessMove>) {
    // If we're at final depth, evaluate.
    if depth == 0 {
        return (evaluate(&p, color), None);
    }

    // Check if there are no moves on the node. If not, retrieve them and add them to the node.
    if node.children.len() == 0 {
        let moves = p.get_moves(color);
        for chess_move in moves {
            node.children.push(Node::new(Some(chess_move), None));
        }
    }

    // Calculate possible moves
    let (mut best_eval, mut best_move) = (f64::NEG_INFINITY, None);
    for child in node.children.iter_mut() {
        // Make the move.
        let child_move = child.last_move.expect("Couldn't get child move!");
        let old_piece = p.board[child_move.o_rank][child_move.o_file];
        let captured_piece = p.board[child_move.n_rank][child_move.n_file];
        p.make_move(&child_move)
            .expect(&format!("Failed to make move {}", child_move));
        // println!("depth {} node", depth);
        let (mut eval, _) = calculate(p, color.opp_color(), depth - 1, -beta, -alpha, child);
        eval = -eval;
        child.eval = Some(eval);
        if eval > best_eval {
            best_eval = eval;
            best_move = Some(child_move);
        }
        best_eval = f64::max(best_eval, eval);

        // Roll back move.
        p.board[child_move.o_rank][child_move.o_file] = old_piece;
        p.board[child_move.n_rank][child_move.n_file] = captured_piece;

        alpha = f64::max(alpha, best_eval);
        if alpha >= beta {
            break;
        }
    }

    // From possible moves, choose optimal move. Return the optimal move with its evaluation.
    sort_moves(&mut node.children);
    (best_eval, best_move)
}

// TODO
fn evaluate(p: &Position, color: Color) -> f64 {
    // For now, let's play like a child. Maximize material.
    let side_sum = p.sum_material(color);

    let opp_sum = p.sum_material(color.opp_color());

    // let central_control = p.central_control(color) * 0.1;

    // let opp_central_control = p.central_control(color.opp_color()) * 0.1;

    // return sideSum - opp_sum + central_control - opp_central_control;
    //println!("Evaluation: {}", side_sum - opp_sum);
    return side_sum - opp_sum;
}

fn sort_moves(moves: &mut Vec<Node>) -> () {
    moves.sort_by(|a, b| match (a.eval, b.eval) {
        (Some(a), Some(b)) => b.partial_cmp(&a).unwrap(),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        _ => Ordering::Equal,
    });
}
