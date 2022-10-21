use super::Player;
use super::board::{ChessBoard, PieceType, ChessPiece, Move};
use super::bitboard::*;
use super::enums::{piece_color::*, end_type::*};
use std::io::{self, Write};
use std::thread;
use work_queue;

const DEPTH_SEARCH: usize = 5;
const SEARCH_TIME: usize = 300;
const NODE_CHILDREN_START_CAPACITY: usize = 50;

const WHITE_PAWN_PLACEMENT_SCORE: [[i32; 8]; 8] = [[0,   5,   5,  0,  5, 10, 50, 0],
                                                   [0,  10,  -5,  0,  5, 10, 50, 0],
                                                   [0,  10, -10,  0, 10, 20, 50, 0],
                                                   [0, -20,   0, 20, 25, 30, 50, 0],
                                                   [0, -20,   0, 20, 25, 30, 50, 0],
                                                   [0,  10, -10,  0, 10, 20, 50, 0],
                                                   [0,  10,  -5,  0,  5, 10, 50, 0],
                                                   [0,   5,   5,  0,  5, 10, 50, 0]];

const BLACK_PAWN_PLACEMENT_SCORE: [[i32; 8]; 8] = [[0, 50, 10,  5,  0,   5,   5, 0],
                                                   [0, 50, 10,  5,  0,  -5,  10, 0],
                                                   [0, 50, 20, 10,  0, -10,  10, 0],
                                                   [0, 50, 30, 25, 20,   0, -20, 0],
                                                   [0, 50, 30, 25, 20,   0, -20, 0],
                                                   [0, 50, 20, 10,  0, -10,  10, 0],
                                                   [0, 50, 10,  5,  5,  -5,  10, 0],
                                                   [0, 50, 10,  5,  5,   5,   5, 0]];

const WHITE_ROOK_PLACEMENT_SCORE: [[i32; 8]; 8] = [[0, -5, -5, -5, -5, -5,  5, 0],
                                                   [0,  0,  0,  0,  0,  0, 10, 0],
                                                   [0,  0,  0,  0,  0,  0, 10, 0],
                                                   [5,  0,  0,  0,  0,  0, 10, 0],
                                                   [5,  0,  0,  0,  0,  0, 10, 0],
                                                   [0,  0,  0,  0,  0,  0, 10, 0],
                                                   [0,  0,  0,  0,  0,  0, 10, 0],
                                                   [0, -5, -5, -5, -5, -5,  5, 0]];

const BLACK_ROOK_PLACEMENT_SCORE: [[i32; 8]; 8] = [[0,  5, -5, -5, -5, -5, -5, 0],
                                                   [0, 10,  0,  0,  0,  0,  0, 0],
                                                   [0, 10,  0,  0,  0,  0,  0, 0],
                                                   [0, 10,  0,  0,  0,  0,  0, 5],
                                                   [0, 10,  0,  0,  0,  0,  0, 5],
                                                   [0, 10,  0,  0,  0,  0,  0, 0],
                                                   [0, 10,  0,  0,  0,  0,  0, 0],
                                                   [0,  5, -5, -5, -5, -5, -5, 0]];

const WHITE_KNIGHT_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-50, -40, -30, -30, -30, -30, -40, -50],
                                                     [-40, -20,   5,   0,   5,   0, -20, -40],
                                                     [-30,   0,  10,  15,  15,  10,   0, -30],
                                                     [-30,   5,  15,  20,  20,  15,   0, -30],
                                                     [-30,   5,  15,  20,  20,  15,   0, -30],
                                                     [-30,   0,  10,  15,  15,  10,   0, -30],
                                                     [-40, -20,   5,   0,   5,   0, -20, -40],
                                                     [-50, -40, -30, -30, -30, -30, -40, -50]];

const BLACK_KNIGHT_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-50, -40, -30, -30, -30, -30, -40, -50],
                                                     [-40, -20,   0,   5,   0,   5, -20, -40],
                                                     [-30,   0,  10,  15,  15,  10,   0, -30],
                                                     [-30,   0,  15,  20,  20,  15,   5, -30],
                                                     [-30,   0,  15,  20,  20,  15,   5, -30],
                                                     [-30,   0,  10,  15,  15,  10,   0, -30],
                                                     [-40, -20,   0,   5,   0,   5, -20, -40],
                                                     [-50, -40, -30, -30, -30, -30, -40, -50]];

const WHITE_BISHOP_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-20, -10, -10, -10, -10, -10, -10, -20],
                                                     [-10,   5,  10,   0,   5,   0,   0, -10],
                                                     [-10,   0,  10,  10,   5,   5,   0, -10],
                                                     [-10,   0,  10,  10,  10,  10,   0, -10],
                                                     [-10,   0,  10,  10,  10,  10,   0, -10],
                                                     [-10,   0,  10,  10,   5,   5,   0, -10],
                                                     [-10,   5,  10,   0,   5,   0,   0, -10],
                                                     [-20, -10, -10, -10, -10, -10, -10, -20]];

const BLACK_BISHOP_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-20, -10, -10, -10, -10, -10, -10, -20],
                                                     [-10,   0,   0,   5,   0,  10,   5, -10],
                                                     [-10,   0,   5,   5,  10,  10,   0, -10],
                                                     [-10,   0,  10,  10,  10,  10,   0, -10],
                                                     [-10,   0,  10,  10,  10,  10,   0, -10],
                                                     [-10,   0,   5,   5,  10,  10,   0, -10],
                                                     [-10,   0,   0,   5,   0,  10,   5, -10],
                                                     [-20, -10, -10, -10, -10, -10, -10, -20]];

const WHITE_QUEEN_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-20, -10, -10, 0, -5, -10, -10, -20],
                                                    [-10,   0,   5, 0,  0,   0,   0,  10],
                                                    [-10,   5,   5, 5,  5,   5,   0, -10],
                                                    [ -5,   0,   5, 5,  5,   5,   0,  -5],
                                                    [ -5,   0,   5, 5,  5,   5,   0,  -5],
                                                    [-10,   0,   5, 5,  5,   5,   0, -10],
                                                    [-10,   0,   5, 0,  0,   0,   0,  10],
                                                    [-20, -10, -10, 0, -5, -10, -10, -20]];

const BLACK_QUEEN_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-20, -10, -10, -5, 0, -10, -10, -20],
                                                    [ 10,   0,   0,  0, 0,   5,   0, -10],
                                                    [-10,   0,   5,  5, 5,   5,   5, -10],
                                                    [ -5,   0,   5,  5, 5,   5,   0,  -5],
                                                    [ -5,   0,   5,  5, 5,   5,   0,  -5],
                                                    [-10,   0,   5,  5, 5,   5,   0, -10],
                                                    [ 10,   0,   0,  0, 0,   0,   0, -10],
                                                    [-20, -10, -10, -5, 0, -10, -10, -20]];

const WHITE_KING_EARLY_PLACEMENT_SCORE: [[i32; 8]; 8] = [[20, 20, -10, -20, -30, -30, -30, -30],
                                                         [30, 20, -20, -30, -40, -40, -40, -40],
                                                         [10,  0, -20, -30, -40, -40, -40, -40],
                                                         [ 0,  0, -20, -40, -50, -50, -50, -50],
                                                         [ 0,  0, -20, -40, -50, -50, -50, -50],
                                                         [10,  0, -20, -30, -40, -40, -40, -40],
                                                         [30, 20, -20, -30, -40, -40, -40, -40],
                                                         [20, 20, -10, -20, -30, -30, -30, -30]];

const BLACK_KING_EARLY_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-30, -30, -30, -30, -20, -10, 20, 20],
                                                         [-40, -40, -40, -40, -30, -20,  0, 10],
                                                         [-40, -40, -40, -40, -30, -20,  0, 10],
                                                         [-50, -50, -50, -50, -40, -20,  0,  0],
                                                         [-50, -50, -50, -50, -40, -20,  0,  0],
                                                         [-40, -40, -40, -40, -30, -20,  0, 10],
                                                         [-40, -40, -40, -40, -30, -20,  0, 10],
                                                         [-30, -30, -30, -30, -20, -10, 20, 20]];

const WHITE_KING_LATE_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-50, -30, -30, -30, -30, -30, -30, -50],
                                                        [-30, -30, -10, -10, -10, -10, -20, -40],
                                                        [-30,   0,  20,  30,  30,  20, -10, -30],
                                                        [-30,   0,  30,  40,  40,  30,   0, -20],
                                                        [-30,   0,  30,  40,  40,  30,   0, -20],
                                                        [-30,   0,  20,  30,  30,  20, -10, -30],
                                                        [-30, -30, -10, -10, -10, -10, -20, -40],
                                                        [-50, -50, -50, -50, -40, -20,   0,   0]];

const BLACK_KING_LATE_PLACEMENT_SCORE: [[i32; 8]; 8] = [[-50, -30, -30, -30, -30, -30, -30, -50],
                                                        [-40, -20, -10, -10, -10, -10, -30, -30],
                                                        [-30, -10,  20,  30,  30,  20,   0, -30],
                                                        [-20,   0,  30,  40,  40,  30,   0, -30],
                                                        [-20,   0,  30,  40,  40,  30,   0, -30],
                                                        [-30, -10,  20,  30,  30,  20,   0, -30],
                                                        [-40, -20, -10, -10, -10, -10, -30, -30],
                                                        [-50, -30, -30, -30, -30, -30, -30, -50]];

fn get_letter(letter: usize) -> char {
    match letter {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        _ => 'h',
    }
}

fn get_number(number: usize) -> char {
    match number {
        0 => '1',
        1 => '2',
        2 => '3',
        3 => '4',
        4 => '5',
        5 => '6',
        6 => '7',
        _ => '8',
    }
}


pub fn player_move(board: &ChessBoard, previous_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, turn: PieceColor, player: &Player) -> String {
    let color_str = match turn {
        PieceColor::White => "White",
        PieceColor::Black => "Black"
    };

    println!("It is {}'s turn! (You)\n", color_str);
    println!("{}", ChessBoard::board_ascii(board, true));
    print!("\nEnter your move: ");
    std::io::stdout().flush();

    let mut inp = String::new();
    io::stdin().read_line(&mut inp);
    
    inp
}

pub type EvaluationFunction = fn(&ChessBoard, Option<&ChessBoard>, &Vec<ChessBoard>, i32) -> i32;

pub fn minimax(board: &ChessBoard, previous_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, turn: PieceColor, player: &Player, eval_func: EvaluationFunction, alpha_beta_pruning: bool, multi_threading: bool) -> String {
    println!("Looking {} moves ahead...", player.moves_ahead);
    let start_time = std::time::Instant::now();

    let mut evaluated_moves: Vec<(i32, Move)> = Vec::new();

    let moveset_board = board.generate_moveset_board(previous_board, turn);

    if !multi_threading {
        for mov in moveset_board.iter() {
            let new_board = board.do_move(mov);
            let mut new_board_history = board_history.clone();
            new_board_history.push(new_board.clone());
            let eval = minimax_helper(&new_board, Some(board), turn.opposite_color(), eval_func, &new_board_history, player.moves_ahead - 1, i32::MIN, i32::MAX, alpha_beta_pruning);
            evaluated_moves.push((eval, mov.clone()));
        }
    } else {
        let threads = num_cpus::get();
        let queue: work_queue::Queue<Move> = work_queue::Queue::new(threads, 128);

        for mov in moveset_board.iter() {
            queue.push(mov.clone());
        }

        let handles: Vec<_> = queue.local_queues().map(|mut local_queue| {
            let board_cp = board.clone();
            let mut board_history_cp = board_history.clone();
            let new_depth = player.moves_ahead - 1;
            std::thread::spawn(move || -> Vec<(i32, Move)> {
                let mut results: Vec<(i32, Move)> = Vec::with_capacity(20);
                while let Some(mov) = local_queue.pop() {
                    let new_board = board_cp.do_move(&mov);
                    board_history_cp.push(new_board.clone());
                    let eval = minimax_helper(&new_board, Some(&board_cp), turn.opposite_color(), eval_func, &board_history_cp, new_depth, i32::MIN, i32::MAX, alpha_beta_pruning);
                    results.push((eval, mov));
                }
                return results;
            })
        }).collect();
    
        for handle in handles {
            let ret = handle.join().unwrap();
            for result in ret.iter() {
                evaluated_moves.push(result.clone());
            }
        }
    }
    

    let init = match turn {
        PieceColor::White => i32::MIN,
        PieceColor::Black => i32::MAX
    };

    let best_move = evaluated_moves.iter().fold((init, None), |a, b| {
        match turn {
            PieceColor::White => if a.0 > b.0 { a } else { (b.0, Some(&b.1)) },
            PieceColor::Black => if a.0 < b.0 { a } else { (b.0, Some(&b.1)) }
        }
    }).1.expect("Minimax did not find any moves!");

    let ((letter_from, number_from), (letter_to, number_to)) = best_move.moves[0];
    let mov_str = format!("{}{} {}{} \n", get_letter(letter_from), get_number(number_from), get_letter(letter_to), get_number(number_to));
    println!("Finished in {} seconds, making the following move: {}", start_time.elapsed().as_millis() as f32 / 1000., mov_str);

    mov_str
}

// alpha min beta max

fn minimax_helper(board: &ChessBoard, previous_board: Option<&ChessBoard>, turn: PieceColor, eval_func: EvaluationFunction, board_history: &Vec<ChessBoard>, depth: i32, alpha: i32, beta: i32, alpha_beta_pruning: bool) -> i32 {
    let maximizing_player = turn == PieceColor::White;
    let moveset_board = board.generate_moveset_board(previous_board, turn);
    let move_count = moveset_board.count_moves();

    if depth == 0 || move_count == 0 {
        return eval_func(board, previous_board, board_history, depth);
    }

    let mut ret_value = if maximizing_player {
        i32::MIN
    } else {
        i32::MAX
    };

    let mut new_alpha = alpha;
    let mut new_beta = beta;

    for mov in moveset_board.iter() {
        let new_board = board.do_move(mov);
        let mut new_board_history = board_history.clone();
        new_board_history.push(new_board.clone());
        let eval = minimax_helper(&new_board, Some(board), turn.opposite_color(), eval_func, &new_board_history, depth - 1, new_alpha, new_beta, alpha_beta_pruning);

        if maximizing_player {
            if eval > ret_value {
                ret_value = eval;
            }
            if eval > new_alpha {
                new_alpha = eval;
            }
        } else {
            if eval < ret_value {
                ret_value = eval;
            }
            if eval < new_beta {
                new_beta = eval;
            }
        }

        if alpha_beta_pruning && new_beta <= new_alpha {
            break;
        }
    }

    return ret_value;
}

// board: &BitBoard, prev_board: Option<&BitBoard>, board_history: &Vec<BitBoard>, depth: i32, constants: &Constants
pub type BitBoardEvaluationFunction = fn(&BitBoard, Option<&BitBoard>, &Vec<BitBoard>, i32, &Constants) -> i32;
type BitBoardMove = ((u64, u64), (u64, u64));

pub fn bitboard_minimax(board: &ChessBoard, previous_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, turn: PieceColor, player: &Player, eval_func: EvaluationFunction, alpha_beta_pruning: bool, multi_threading: bool, constants: Constants, other_eval: BitBoardEvaluationFunction) -> String {
    println!("Looking {} moves ahead...", player.moves_ahead);
    let start_time = std::time::Instant::now();

    let board = board_to_bitboard(board);
    let previous_board = match previous_board {
        Some(board) => Some(board_to_bitboard(board)),
        None => None
    };
    let board_history: Vec<[u64; 12]> = board_history.iter().map(|board| board_to_bitboard(board)).collect();

    let mut evaluated_moves: Vec<(i32, BitBoardMove)> = Vec::new();

    let moves = generate_possible_moves(&board, previous_board.as_ref(), turn, &constants);

    if !multi_threading {
        for (mov, new_bitboard) in moves {
            let mut new_board_history = board_history.clone();
            new_board_history.push(new_bitboard.clone());
            let eval = bitboard_minimax_helper(&new_bitboard, Some(&board), turn.opposite_color(), eval_func, &new_board_history, player.moves_ahead - 1, i32::MIN, i32::MAX, alpha_beta_pruning, &constants, other_eval);
            evaluated_moves.push((eval, mov));
        }
    } else {


        let threads = num_cpus::get();
        let queue: work_queue::Queue<(BitBoardMove, [u64; 12])> = work_queue::Queue::new(threads, 128);

        for item in moves {
            queue.push(item.clone());
        }

        let handles: Vec<_> = queue.local_queues().map(|mut local_queue| {
            let board_cp = board.clone();
            let mut board_history_cp = board_history.clone();
            let new_depth = player.moves_ahead - 1;
            let constants_cp = constants.clone();
            std::thread::spawn(move || -> Vec<(i32, BitBoardMove)> {
                let mut results: Vec<(i32, BitBoardMove)> = Vec::with_capacity(20);
                while let Some((mov, new_board)) = local_queue.pop() {
                    board_history_cp.push(new_board.clone());
                    let eval = bitboard_minimax_helper(&new_board, Some(&board_cp), turn.opposite_color(), eval_func, &board_history_cp, new_depth, i32::MIN, i32::MAX, alpha_beta_pruning, &constants_cp, other_eval);
                    results.push((eval, mov));
                }
                return results;
            })
        }).collect();
    
        for handle in handles {
            let ret = handle.join().unwrap();
            for result in ret.iter() {
                evaluated_moves.push(result.clone());
            }
        }
    }
    

    let init = match turn {
        PieceColor::White => i32::MIN,
        PieceColor::Black => i32::MAX
    };

    let best_move = evaluated_moves.iter().fold((init, None), |a, b| {
        match turn {
            PieceColor::White => if a.0 > b.0 { a } else { (b.0, Some(&b.1)) },
            PieceColor::Black => if a.0 < b.0 { a } else { (b.0, Some(&b.1)) }
        }
    }).1.expect("Minimax did not find any moves!");

    let ((letter_from, number_from), (letter_to, number_to)) = best_move;
    let mov_str = format!("{}{} {}{} \n", get_letter(*letter_from as usize), get_number(*number_from as usize), get_letter(*letter_to as usize), get_number(*number_to as usize));
    println!("Finished in {} seconds, making the following move: {}", start_time.elapsed().as_millis() as f32 / 1000., mov_str);

    mov_str
}

fn bitboard_minimax_helper(board: &BitBoard, previous_board: Option<&BitBoard>, turn: PieceColor, eval_func: EvaluationFunction, board_history: &Vec<BitBoard>, depth: i32, alpha: i32, beta: i32, alpha_beta_pruning: bool, constants: &Constants, other_eval: BitBoardEvaluationFunction) -> i32 {
    let maximizing_player = turn == PieceColor::White;
    let moves = generate_possible_moves(&board, previous_board, turn, &constants);

    if depth == 0 || moves.len() == 0 {
        return other_eval(board, previous_board, board_history, depth, constants);
    }

    let mut ret_value = if maximizing_player {
        i32::MIN
    } else {
        i32::MAX
    };

    let mut new_alpha = alpha;
    let mut new_beta = beta;

    for (_, new_bitboard) in moves {
        let mut new_board_history = board_history.clone();
        new_board_history.push(new_bitboard.clone());
        let eval = bitboard_minimax_helper(&new_bitboard, Some(board), turn.opposite_color(), eval_func, &new_board_history, depth - 1, new_alpha, new_beta, alpha_beta_pruning, constants, other_eval);

        if maximizing_player {
            if eval > ret_value {
                ret_value = eval;
            }
            if eval > new_alpha {
                new_alpha = eval;
            }
        } else {
            if eval < ret_value {
                ret_value = eval;
            }
            if eval < new_beta {
                new_beta = eval;
            }
        }

        if alpha_beta_pruning && new_beta <= new_alpha {
            break;
        }
    }

    return ret_value;
}

fn simple_board_evaluation(board: &ChessBoard, prev_board: Option<&ChessBoard>) -> i32 {
    match &board.check_for_game_end(prev_board, PieceColor::White) {
        EndType::Checkmate => {
            return <i32>::min_value() + 1;
        },
        _ => ()
    }

    match &board.check_for_game_end(prev_board, PieceColor::Black) {
        EndType::Checkmate => {
            return <i32>::max_value() - 1;
        },
        _ => ()
    }

    let get_piece_value = |piece: PieceType| {
        match piece {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Queen => 9,
            PieceType::King => 0
        }
    };
    
    let mut value = 0;
    for i in 0..8 {
        for j in 0..8 {
            if let Ok(Some(piece)) = board.get_piece(i, j) {
                match piece.color {
                    PieceColor::White => {
                        value += get_piece_value(piece.typ);
                    },
                    PieceColor::Black => {
                        value -= get_piece_value(piece.typ);
                    }
                }
            }
        }
    }

    value
}

pub fn simple_board_evaluation_with_position(board: &ChessBoard, prev_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, depth: i32) -> i32 {
    match &board.check_for_game_end(prev_board, PieceColor::White) {
        EndType::Checkmate => {
            return <i32>::min_value() / 2 + depth;
        },
        EndType::Tie => {
            return 0;
        },
        _ => ()
    }

    match &board.check_for_game_end(prev_board, PieceColor::Black) {
        EndType::Checkmate => {
            return <i32>::max_value() / 2 - depth;
        },
        EndType::Tie => {
            return 0;
        },
        _ => ()
    }

    match &board.check_repetition(board_history) {
        EndType::Tie => {
            return 0;
        },
        _ => ()
    }

    let get_piece_value = |piece: PieceType| {
        match piece {
            PieceType::Pawn => 100,
            PieceType::Rook => 500,
            PieceType::Bishop => 300,
            PieceType::Knight => 300,
            PieceType::Queen => 900,
            PieceType::King => 0
        }
    };

    let get_piece_position_value = |piece: &ChessPiece, letter: usize, number: usize| {
        match piece.color {
            PieceColor::White => match piece.typ {
                                    PieceType::Pawn => WHITE_PAWN_PLACEMENT_SCORE[letter][number],
                                    PieceType::Rook => WHITE_ROOK_PLACEMENT_SCORE[letter][number],
                                    PieceType::Bishop => WHITE_BISHOP_PLACEMENT_SCORE[letter][number],
                                    PieceType::Knight => WHITE_KNIGHT_PLACEMENT_SCORE[letter][number],
                                    PieceType::Queen => WHITE_QUEEN_PLACEMENT_SCORE[letter][number],
                                    PieceType::King => WHITE_KING_EARLY_PLACEMENT_SCORE[letter][number]
                                },
            PieceColor::Black => match piece.typ {
                                    PieceType::Pawn => BLACK_PAWN_PLACEMENT_SCORE[letter][number],
                                    PieceType::Rook => BLACK_ROOK_PLACEMENT_SCORE[letter][number],
                                    PieceType::Bishop => BLACK_BISHOP_PLACEMENT_SCORE[letter][number],
                                    PieceType::Knight => BLACK_KNIGHT_PLACEMENT_SCORE[letter][number],
                                    PieceType::Queen => BLACK_QUEEN_PLACEMENT_SCORE[letter][number],
                                    PieceType::King => BLACK_KING_EARLY_PLACEMENT_SCORE[letter][number]
                                }
        }
    };
    
    let mut value = 0;
    for i in 0..8 {
        for j in 0..8 {
            if let Ok(Some(piece)) = board.get_piece(i, j) {
                match piece.color {
                    PieceColor::White => {
                        value += get_piece_value(piece.typ);
                        value += get_piece_position_value(&piece, i as usize, j as usize);
                    },
                    PieceColor::Black => {
                        value -= get_piece_value(piece.typ);
                        value -= get_piece_position_value(&piece, i as usize, j as usize);
                    }
                }
            }
        }
    }

    value
}

pub fn simple_board_evaluation_with_position_bitboard(board: &BitBoard, prev_board: Option<&BitBoard>, board_history: &Vec<BitBoard>, depth: i32, constants: &Constants) -> i32 {
    match bitboard_check_game_end(&board, prev_board, PieceColor::White, &constants) {
        EndType::Checkmate => {
            return <i32>::min_value() / 2 + depth;
        },
        EndType::Tie => {
            return 0;
        },
        _ => ()
    }

    match bitboard_check_game_end(&board, prev_board, PieceColor::Black, &constants) {
        EndType::Checkmate => {
            return <i32>::max_value() / 2 - depth;
        },
        EndType::Tie => {
            return 0;
        },
        _ => ()
    }

    // match &board.check_repetition(board_history) {
    //     EndType::Tie => {
    //         return 0;
    //     },
    //     _ => ()
    // }
    
    let mut value = 0;
    for i in 0..8 {
        for j in 0..8 {
            let num = 1 << pos_to_num(i, j);
            let (letter, number) = (i as usize, j as usize);
            if board[PieceNum::WhitePawn as usize] & num == num {
                value += 100;
                value += WHITE_PAWN_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::WhiteRook as usize] & num == num {
                value += 500;
                value += WHITE_ROOK_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::WhiteKnight as usize] & num == num {
                value += 300;
                value += WHITE_BISHOP_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::WhiteBishop as usize] & num == num {
                value += 300;
                value += WHITE_KNIGHT_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::WhiteQueen as usize] & num == num {
                value += 900;
                value += WHITE_QUEEN_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::WhiteKing as usize] & num == num {
                value += 0;
                value += WHITE_KING_EARLY_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::BlackPawn as usize] & num == num {
                value -= 100;
                value -= BLACK_PAWN_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::BlackRook as usize] & num == num {
                value -= 500;
                value -= BLACK_ROOK_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::BlackKnight as usize] & num == num {
                value -= 300;
                value -= BLACK_KNIGHT_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::BlackBishop as usize] & num == num {
                value -= 300;
                value -= BLACK_BISHOP_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::BlackQueen as usize] & num == num {
                value -= 900;
                value -= BLACK_QUEEN_PLACEMENT_SCORE[letter][number];
            } else if board[PieceNum::BlackKing as usize] & num == num {
                value -= 0;
                value -= BLACK_KING_EARLY_PLACEMENT_SCORE[letter][number];
            }
        }
    }

    value
}