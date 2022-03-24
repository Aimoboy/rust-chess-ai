use super::board::*;
use std::io::{self, Write};

const DEPTH_SEARCH: usize = 4;
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


pub fn player_move(board: &ChessBoard, turn: PieceColor) -> String {
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

pub fn mini_max_ai(board: &ChessBoard, turn: PieceColor) -> String {
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

    let tree = generate_board_tree(board, turn);
    let maxi_player = if turn == PieceColor::White { true } else { false };
    let ((letter_from, number_from), (letter_to, number_to)) = mini_max(&tree, None, maxi_player, simple_board_evaulation).1.unwrap().moves[0];
    let mov_str = format!("{}{} {}{} \n", get_letter(letter_from), get_number(number_from), get_letter(letter_to), get_number(number_to));
    println!("{}", mov_str);
    mov_str
}

pub fn alpha_beta_pruning_ai(board: &ChessBoard, turn: PieceColor) -> String {
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
    println!("Generating move tree with depth {} using max {} seconds", DEPTH_SEARCH, SEARCH_TIME);
    let start = std::time::Instant::now();
    let tree = generate_board_tree(board, turn);
    let maxi_player = if turn == PieceColor::White { true } else { false };
    println!("Finished generating the tree in {} seconds (nodes: {}, leaves: {}, height: {}), starting search", start.elapsed().as_secs(), tree.count_nodes(), tree.count_leaves(), tree.get_height());
    let start = std::time::Instant::now();
    let ((letter_from, number_from), (letter_to, number_to)) = alpha_beta_pruning(&tree, None, maxi_player, <i32>::min_value(), <i32>::max_value(), 0, simple_board_evaulation_with_position).1.unwrap().moves[0];
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

fn alpha_beta_pruning(node: &Node, prev_board: Option<&ChessBoard>, max: bool, alpha: i32, beta: i32, depth: i32, evaluate: fn (board: &ChessBoard, prev_board: Option<&ChessBoard>, depth: i32) -> i32) -> (i32, Option<Move>) {
    let return_mov = match node.mov.as_ref() {
        Some(mov) => Some(mov.clone()),
        None => None
    };

    if node.is_leaf() {
        let eval = evaluate(&node.data, prev_board, depth);
        return (eval, return_mov);
    }

    if max {
        let mut max_eval = <i32>::min_value();
        
        let mut mov = &None;
        let mut new_alpha = alpha;
        for child in &node.children {
            let eval = alpha_beta_pruning(child, Some(&node.data), false, new_alpha, beta, depth + 1, evaluate).0;
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
            let eval = alpha_beta_pruning(child, Some(&node.data), true, alpha, new_beta, depth + 1, evaluate).0;
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

pub fn generate_board_tree(board: &ChessBoard, turn: PieceColor) -> Node {
    let now = std::time::Instant::now();
    let mut root = Node::new(board.clone(), None);
    generate_children(&mut root, None, turn, DEPTH_SEARCH, now);
    root
}

fn generate_children(root: &mut Node, root_root: Option<&ChessBoard>, turn: PieceColor, depth: usize, start: std::time::Instant) {
    // println!("{}", start.elapsed().as_secs());
    if start.elapsed().as_secs() as usize >= SEARCH_TIME {
        // println!("test2");
        return;
    }

    let b = &root.data;

    let moves = ChessBoard::generate_moveset_board(b, root_root, turn);

    for i in 0..8 {
        for j in 0..8 {
            let vec = &moves[i][j];
            for mov in vec {
                root.children.push(Node::new(b.clone().do_move(&mov), Some(mov.clone())));
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

fn simple_board_evaulation(board: &ChessBoard, prev_board: Option<&ChessBoard>) -> i32 {
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

fn simple_board_evaulation_with_position(board: &ChessBoard, prev_board: Option<&ChessBoard>, depth: i32) -> i32 {
    match &board.check_for_game_end(prev_board, PieceColor::White) {
        EndType::Checkmate => {
            return <i32>::min_value() / 2 + depth;
        },
        _ => ()
    }

    match &board.check_for_game_end(prev_board, PieceColor::Black) {
        EndType::Checkmate => {
            return <i32>::max_value() / 2 - depth;
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