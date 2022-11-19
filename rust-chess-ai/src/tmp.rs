use crate::traits::chess_board_contract::ChessBoardContract;

use super::Player;
use super::enums::{piece_color::PieceColor, end_type::EndType, chess_error::ChessError, piece_type::PieceType};

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


// fn simple_board_evaluation(board: &ChessBoard, prev_board: Option<&ChessBoard>) -> i32 {
//     match &board.check_for_game_end(prev_board, PieceColor::White) {
//         EndType::Checkmate => {
//             return <i32>::min_value() + 1;
//         },
//         _ => ()
//     }

//     match &board.check_for_game_end(prev_board, PieceColor::Black) {
//         EndType::Checkmate => {
//             return <i32>::max_value() - 1;
//         },
//         _ => ()
//     }

//     let get_piece_value = |piece: PieceType| {
//         match piece {
//             PieceType::Pawn => 1,
//             PieceType::Rook => 5,
//             PieceType::Bishop => 3,
//             PieceType::Knight => 3,
//             PieceType::Queen => 9,
//             PieceType::King => 0
//         }
//     };
    
//     let mut value = 0;
//     for i in 0..8 {
//         for j in 0..8 {
//             if let Ok(Some(piece)) = board.get_piece(i, j) {
//                 match piece.color {
//                     PieceColor::White => {
//                         value += get_piece_value(piece.typ);
//                     },
//                     PieceColor::Black => {
//                         value -= get_piece_value(piece.typ);
//                     }
//                 }
//             }
//         }
//     }

//     value
// }

// pub fn simple_board_evaluation_with_position(board: &ChessBoard, prev_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, depth: i32) -> i32 {
//     match &board.check_for_game_end(prev_board, PieceColor::White) {
//         EndType::Checkmate => {
//             return <i32>::min_value() / 2 + depth;
//         },
//         EndType::Tie => {
//             return 0;
//         },
//         _ => ()
//     }

//     match &board.check_for_game_end(prev_board, PieceColor::Black) {
//         EndType::Checkmate => {
//             return <i32>::max_value() / 2 - depth;
//         },
//         EndType::Tie => {
//             return 0;
//         },
//         _ => ()
//     }

//     match &board.check_repetition(board_history) {
//         EndType::Tie => {
//             return 0;
//         },
//         _ => ()
//     }

//     let get_piece_value = |piece: PieceType| {
//         match piece {
//             PieceType::Pawn => 100,
//             PieceType::Rook => 500,
//             PieceType::Bishop => 300,
//             PieceType::Knight => 300,
//             PieceType::Queen => 900,
//             PieceType::King => 0
//         }
//     };

//     let get_piece_position_value = |piece: &ChessPiece, letter: usize, number: usize| {
//         match piece.color {
//             PieceColor::White => match piece.typ {
//                                     PieceType::Pawn => WHITE_PAWN_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Rook => WHITE_ROOK_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Bishop => WHITE_BISHOP_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Knight => WHITE_KNIGHT_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Queen => WHITE_QUEEN_PLACEMENT_SCORE[letter][number],
//                                     PieceType::King => WHITE_KING_EARLY_PLACEMENT_SCORE[letter][number]
//                                 },
//             PieceColor::Black => match piece.typ {
//                                     PieceType::Pawn => BLACK_PAWN_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Rook => BLACK_ROOK_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Bishop => BLACK_BISHOP_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Knight => BLACK_KNIGHT_PLACEMENT_SCORE[letter][number],
//                                     PieceType::Queen => BLACK_QUEEN_PLACEMENT_SCORE[letter][number],
//                                     PieceType::King => BLACK_KING_EARLY_PLACEMENT_SCORE[letter][number]
//                                 }
//         }
//     };
    
//     let mut value = 0;
//     for i in 0..8 {
//         for j in 0..8 {
//             if let Ok(Some(piece)) = board.get_piece(i, j) {
//                 match piece.color {
//                     PieceColor::White => {
//                         value += get_piece_value(piece.typ);
//                         value += get_piece_position_value(&piece, i as usize, j as usize);
//                     },
//                     PieceColor::Black => {
//                         value -= get_piece_value(piece.typ);
//                         value -= get_piece_position_value(&piece, i as usize, j as usize);
//                     }
//                 }
//             }
//         }
//     }

//     value
// }
