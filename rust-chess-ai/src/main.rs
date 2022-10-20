extern crate work_queue;
extern crate num_cpus;

mod game;
mod tests;
mod turn_functions;

mod board_types {
    pub mod board;
    pub mod bitboard;
}

mod enums {
    pub mod piece_color;
    pub mod end_type;
    pub mod board_type;
}

use board_types::board::ChessBoard;
use game::Game;
use board_types::board;
use board_types::bitboard;
use enums::piece_color::PieceColor;
use enums::board_type::BoardType;
use turn_functions::{minimax, player_move, EvaluationFunction, simple_board_evaluation_with_position};

pub struct Player {
    turn_function: Box<dyn Fn(&ChessBoard, Option<&ChessBoard>, &Vec<ChessBoard>, PieceColor, &Player) -> String>,
    moves_ahead: i32,
    board_type: BoardType
}

impl Player {
    pub fn human_player() -> Self {
        Self {
            turn_function: Box::new(player_move),
            moves_ahead: 0,
            board_type: BoardType::Standard
        }
    }

    pub fn minimax_bot(moves_ahead: i32, board_type: BoardType, eval_func: EvaluationFunction, alpha_beta_pruning: bool, multi_threading: bool) -> Self {
        Self {
            turn_function: {
                Box::new(move |board: &ChessBoard, previous_board: Option<&ChessBoard>, board_history: &Vec<ChessBoard>, turn: PieceColor, player: &Player| -> String {
                    minimax(board, previous_board, board_history, turn, player, eval_func, alpha_beta_pruning, multi_threading)
                })
            },
            moves_ahead: moves_ahead,
            board_type: board_type
        }
    }
}


fn main() {
    let new_game = Game::new();

    let white_player = Player::human_player();

    let black_player = Player::minimax_bot(6, BoardType::Standard, simple_board_evaluation_with_position, true, true);

    Game::run(new_game, white_player, black_player);
}
