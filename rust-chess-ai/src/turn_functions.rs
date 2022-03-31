use super::board::*;
use super::piece_color::*;
use std::io::{self, Write};
use std::thread;

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


pub fn player_move(board: &ChessBoard, previous_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, turn: PieceColor, moves_ahead: i32) -> String {
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

pub fn alpha_beta_pruning_ai_tree_generate_with_threads(board: &ChessBoard, previous_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, turn: PieceColor, moves_ahead: i32) -> String {
    let get_letter = |letter: usize| {
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
    };

    let get_number = |number: usize| {
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
    };

    println!("Looking {} moves ahead...", moves_ahead);
    let start_time = std::time::Instant::now();
    let moveset_board = board.generate_moveset_board(previous_board, turn);
    // let mut count = 0;

    // for i in 0..8 {
    //     for j in 0..8 {
    //         count += moveset_board[i][j].len();
    //     }
    // }
    // println!("{:?}", moveset_board);
    // println!("{:?}", count);
    // return "a7 a6 \n".to_string();
    let mut possible_moves = Vec::with_capacity(50);

    for i in 0..8 {
        for j in 0..8 {
            for mov in &moveset_board[i][j] {
                possible_moves.push(mov);
            }
        }
    }

    let mut thread_handles = Vec::with_capacity(50);

    let opposite_turn = match turn {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White
    };

    for mov in possible_moves {
        let prev_board_cp = if let Some(board) = previous_board {
            Some(board.clone())
        } else {
            None
        };
        let mov_cp = mov.clone();
        let new_board = board.do_move(mov);
        let board_history_cp = board_history.clone();
        thread_handles.push(thread::spawn(move || -> (i32, Move) {
            (thread_work(new_board, prev_board_cp, opposite_turn, moves_ahead, &board_history_cp), mov_cp)
        }));
    }

    let mut thread_results = Vec::with_capacity(50);

    // for mov in possible_moves {
    //     let prev_board_cp = if let Some(board) = previous_board {
    //         Some(board.clone())
    //     } else {
    //         None
    //     };
    //     let mov_cp = mov.clone();
    //     let new_board = board.do_move(mov);
    //     thread_results.push((thread_work(new_board, prev_board_cp, opposite_turn, moves_ahead), mov_cp));
    // }

    for handle in thread_handles {
        let res = handle.join().unwrap();
        thread_results.push(res);
    }

    let mut best_move = 0;

    for i in 0..thread_results.len() {
        if turn == PieceColor::White && thread_results[i].0 > thread_results[best_move].0 {
            best_move = i;
        } else if turn == PieceColor::Black && thread_results[i].0 < thread_results[best_move].0 {
            best_move = i;
        }
    }

    let mov = &thread_results[best_move].1;
    let ((letter_from, number_from), (letter_to, number_to)) = mov.moves[0];

    let mov_str = format!("{}{} {}{} \n", get_letter(letter_from), get_number(number_from), get_letter(letter_to), get_number(number_to));
    println!("Finished in {} seconds, making the following move: {}", start_time.elapsed().as_millis() as f32 / 1000., mov_str);
    mov_str
}

fn thread_work(board: ChessBoard, prev_board: Option<ChessBoard>, turn: PieceColor, moves_ahead: i32, board_history: &Vec<ChessBoard>) -> i32 {
    let maximizing_player = match turn {
        PieceColor::White => true,
        PieceColor::Black => false
    };
    thread_work_helper(board, prev_board.as_ref(), maximizing_player, <i32>::min_value(), <i32>::max_value(), 1, simple_board_evaluation_with_position, moves_ahead, board_history)
}

fn thread_work_helper(board: ChessBoard, prev_board: Option<&ChessBoard>, maximizing_player: bool, mut alpha: i32, mut beta: i32, depth: i32, evaluate: fn (board: &ChessBoard, prev_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, depth: i32) -> i32, moves_ahead: i32, board_history: &Vec<ChessBoard>) -> i32 {
    let move_board = if maximizing_player {
        board.generate_moveset_board(prev_board, PieceColor::White)
    } else {
        board.generate_moveset_board(prev_board, PieceColor::Black)
    };

    let mut move_count = 0;

    for i in 0..8 {
        for j in 0..8 {
            move_count += move_board[i][j].len();
        }
    }

    if move_count == 0 || depth >= moves_ahead as i32 {
        return evaluate(&board, prev_board, board_history, depth);
    }

    if maximizing_player {
        let mut max_eval = <i32>::min_value();

        'first_max: for i in 0..8 {
            for j in 0..8 {
                for mov in &move_board[i][j] {
                    let child_board = board.do_move(mov);
                    let eval = thread_work_helper(child_board, Some(&board), false, alpha, beta, depth + 1, evaluate, moves_ahead, board_history);
                    max_eval = std::cmp::max(max_eval, eval);
                    alpha = std::cmp::max(alpha, eval);
                    if beta <= alpha {
                        break 'first_max;
                    }
                }
            }
        }

        return max_eval;
    } else {
        let mut min_eval = <i32>::max_value();

        'first_min: for i in 0..8 {
            for j in 0..8 {
                for mov in &move_board[i][j] {
                    let child_board = board.do_move(mov);
                    let eval = thread_work_helper(child_board, Some(&board), true, alpha, beta, depth + 1, evaluate, moves_ahead, board_history);
                    min_eval = std::cmp::min(min_eval, eval);
                    beta = std::cmp::min(beta, eval);
                    if beta <= alpha {
                        break 'first_min;
                    }
                }
            }
        }

        return min_eval;
    }
}

pub fn mini_max_ai(board: &ChessBoard, turn: PieceColor, moves_ahead: i32) -> String {
    let get_letter = |letter: usize| {
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
    };

    let get_number = |number: usize| {
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
    };

    let tree = generate_board_tree(board, turn, moves_ahead);
    let maxi_player = if turn == PieceColor::White { true } else { false };
    let ((letter_from, number_from), (letter_to, number_to)) = mini_max(&tree, None, maxi_player, simple_board_evaluation).1.unwrap().moves[0];
    let mov_str = format!("{}{} {}{} \n", get_letter(letter_from), get_number(number_from), get_letter(letter_to), get_number(number_to));
    println!("{}", mov_str);
    mov_str
}

pub fn alpha_beta_pruning_ai(board: &ChessBoard, prev_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, turn: PieceColor, moves_ahead: i32) -> String {
    let get_letter = |letter: usize| {
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
    };

    let get_number = |number: usize| {
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
    };

    println!("{}", board.board_ascii(true));
    println!("Generating move tree with depth {} using max {} seconds", moves_ahead, SEARCH_TIME);
    let start = std::time::Instant::now();
    let tree = generate_board_tree(board, turn, moves_ahead);
    let maxi_player = if turn == PieceColor::White { true } else { false };
    println!("Finished generating the tree in {} seconds (nodes: {}, leaves: {}, height: {}), starting search", start.elapsed().as_secs(), tree.count_nodes(), tree.count_leaves(), tree.get_height());
    let start = std::time::Instant::now();
    let ((letter_from, number_from), (letter_to, number_to)) = alpha_beta_pruning(&tree, prev_board, maxi_player, <i32>::min_value(), <i32>::max_value(), 0, simple_board_evaluation_with_position, board_history).1.unwrap().moves[0];
    let mov_str = format!("{}{} {}{} \n", get_letter(letter_from), get_number(number_from), get_letter(letter_to), get_number(number_to));
    println!("Finished searching in {} seconds, making the following move: {}", start.elapsed().as_secs(), mov_str);
    mov_str
}

fn mini_max(node: &Node, prev_board: Option<&ChessBoard>, max: bool, evaluate: fn (board: &ChessBoard, prev_board: Option<&ChessBoard>) -> i32) -> (i32, Option<Move>) {
    let return_mov = match node.mov.as_ref() {
        Some(mov) => Some(mov.clone()),
        None => None
    };

    if node.is_leaf() {
        let eval = evaluate(&node.data, prev_board);
        return (eval, return_mov);
    }

    if max {
        let mut max_eval = <i32>::min_value();
        
        let mut mov = &None;
        for child in &node.children {
            let eval = mini_max(child, Some(&node.data), false, evaluate).0;
            if eval > max_eval {
                max_eval = eval;
                mov = &child.mov;
            }
        }
        
        if return_mov.is_none() {
            return (max_eval, Some(mov.as_ref().unwrap().clone()));
        }
        return (max_eval, return_mov);
    } else {
        let mut min_eval = <i32>::max_value();
        let mut mov = &None;
        for child in &node.children {
            let eval = mini_max(child, Some(&node.data), true, evaluate).0;
            if eval < min_eval {
                min_eval = eval;
                mov = &child.mov;
            }
        }

        if return_mov.is_none() {
            return (min_eval, Some(mov.as_ref().unwrap().clone()));
        }
        return (min_eval, return_mov);
    }
}

fn alpha_beta_pruning(node: &Node, prev_board: Option<&ChessBoard>, max: bool, alpha: i32, beta: i32, depth: i32, evaluate: fn (board: &ChessBoard, prev_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, depth: i32) -> i32, board_history: &Vec<ChessBoard>) -> (i32, Option<Move>) {
    let return_mov = match node.mov.as_ref() {
        Some(mov) => Some(mov.clone()),
        None => None
    };

    if node.is_leaf() {
        let eval = evaluate(&node.data, prev_board, board_history, depth);
        return (eval, return_mov);
    }

    if max {
        let mut max_eval = <i32>::min_value();
        
        let mut mov = &None;
        let mut new_alpha = alpha;
        for child in &node.children {
            let eval = alpha_beta_pruning(child, Some(&node.data), false, new_alpha, beta, depth + 1, evaluate,  board_history).0;
            if eval > max_eval {
                max_eval = eval;
                mov = &child.mov;
            }
            if eval > new_alpha {
                new_alpha = eval;
            }
            if beta <= new_alpha {
                break;
            }
        }
        
        if return_mov.is_none() {
            return (max_eval, Some(mov.as_ref().unwrap().clone()));
        }
        return (max_eval, return_mov);
    } else {
        let mut min_eval = <i32>::max_value();
        let mut mov = &None;
        let mut new_beta = beta;
        for child in &node.children {
            let eval = alpha_beta_pruning(child, Some(&node.data), true, alpha, new_beta, depth + 1, evaluate, board_history).0;
            if eval < min_eval {
                min_eval = eval;
                mov = &child.mov;
            }
            if eval < new_beta {
                new_beta = eval;
            }
            if new_beta <= alpha {
                break;
            }
        }

        if return_mov.is_none() {
            return (min_eval, Some(mov.as_ref().unwrap().clone()));
        }
        return (min_eval, return_mov);
    }
}

pub fn generate_board_tree(board: &ChessBoard, turn: PieceColor, moves_ahead: i32) -> Node {
    let now = std::time::Instant::now();
    let mut root = Node::new(board.clone(), None);
    generate_children(&mut root, None, turn, moves_ahead, now);
    root
}

fn generate_children(root: &mut Node, root_root: Option<&ChessBoard>, turn: PieceColor, depth: i32, start: std::time::Instant) {
    if start.elapsed().as_secs() as usize >= SEARCH_TIME {
        return;
    }

    let b = &root.data;

    let moves = ChessBoard::generate_moveset_board(b, root_root, turn);

    for i in 0..8 {
        for j in 0..8 {
            let vec = &moves[i][j];
            for mov in vec {
                root.children.push(Node::new(b.do_move(&mov), Some(mov.clone())));
            }
        }
    }

    let opponent_color = match turn {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White
    };

    if depth > 1 {
        for node in &mut root.children {
            generate_children(node, Some(b), opponent_color, depth - 1, start);
        }
    }
}


pub struct Node {
    pub data: ChessBoard,
    mov: Option<Move>,
    pub children: Vec<Node>
}

impl Node {
    fn new(data: ChessBoard, mov: Option<Move>) -> Self {
        Self {
            data,
            mov,
            children: Vec::with_capacity(NODE_CHILDREN_START_CAPACITY)
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn count_nodes(&self) -> usize {
        let mut count = 0;

        for node in &self.children {
            count += node.count_nodes();
        }

        count + 1
    }

    pub fn count_leaves(&self) -> usize {
        if self.is_leaf() {
            return 1;
        }

        let mut count = 0;

        for node in &self.children {
            count += node.count_leaves();
        }

        count
    }

    pub fn get_height(&self) -> usize {
        if self.is_leaf() {
            return 0;
        }

        let mut largest_height = 0;

        for node in &self.children {
            largest_height = std::cmp::max(largest_height, node.get_height() + 1);
        }

        largest_height
    }
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

fn simple_board_evaluation_with_position(board: &ChessBoard, prev_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, depth: i32) -> i32 {
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