extern crate work_queue;
extern crate num_cpus;
extern crate rustc_hash;

mod game;
mod tmp;
mod functions;

mod board_types {
    pub mod normalboard;
    pub mod bitboard;
}

mod enums {
    pub mod piece_color;
    pub mod end_type;
    pub mod board_type;
    pub mod chess_error;
    pub mod piece_num;
    pub mod piece_type;
}

mod traits {
    pub mod chess_board_contract;
}

mod turn_functions {
    pub mod player_move;
    pub mod minimax_move;
}

mod evaluation_functions {
    pub mod board_piece_evaluation;
}

mod tests {
    mod unit_tests {
        pub mod normalboard_tests;
        pub mod bitboard_tests;
    }
}





use crate::enums::{
    piece_color::PieceColor,
    chess_error::ChessError
};

use crate::turn_functions::{
    player_move::player_move
};

use crate::evaluation_functions::{
    board_piece_evaluation::board_piece_evaluation
};

use board_types::normalboard::NormalBoard;
use traits::{
    chess_board_contract::ChessBoardContract
};

use board_types::bitboard::{Constants, BitBoard};
use turn_functions::minimax_move::minimax_move;
use crate::game::Game;


pub type EvaluationFunction<T: ChessBoardContract> = fn(&T, Option<&T>, &Vec<T>, i32, &Constants) -> Result<i32, ChessError>;

pub struct Player<T: 'static + ChessBoardContract> {
    turn_function: Box<dyn Fn(&T, Option<&T>, &Vec<T>, PieceColor, &Player<T>, &Constants) -> Result<String, ChessError>>,
    moves_ahead: i32
}

impl<T: 'static + ChessBoardContract + Clone + Send + Sync> Player<T> {
    pub fn human_player() -> Self {
        Self {
            turn_function: Box::new(player_move),
            moves_ahead: 0
        }
    }

    pub fn minimax_bot(moves_ahead: i32, eval_func: EvaluationFunction<T>, alpha_beta_pruning: bool, multi_threading: bool) -> Self {
        Self {
            turn_function: {
                Box::new(move |board: &T, previous_board: Option<&T>, board_history: &Vec<T>, turn: PieceColor, player: &Player<T>, constants: &Constants| -> Result<String, ChessError> {
                    minimax_move(board, previous_board, board_history, turn, player, eval_func, constants, alpha_beta_pruning, multi_threading)
                })
            },
            moves_ahead: moves_ahead
        }
    }
}

fn main() {

    let new_game: Game<BitBoard> = Game::new();

    // let white_player = Player::minimax_bot(3, board_piece_evaluation, true, false);
    let white_player = Player::human_player();

    let black_player = Player::minimax_bot(5, board_piece_evaluation, true, true);

    let res = Game::run(new_game, white_player, black_player);

    match res {
        Ok(_) => print!("Good!"),
        Err(err) => println!("{:?}", err)
    }

}
