use std::marker::PhantomData;

use crate::{
    Player,
    enums::{
        piece_color::PieceColor,
        end_type::EndType,
        chess_error::ChessError
    },
    traits::{
        chess_board_contract::ChessBoardContract
    },
    board_types::bitboard::Constants
};

use std::sync::Arc;

const BOARD_HISTORY_START_CAPACITY: usize = 100;

pub struct Game<T> {
    board_history: Vec<T>,
    turn: PieceColor,
    constants: Constants
}

impl<T: ChessBoardContract + Clone> Game<T> {
    pub fn new() -> Self {
        let mut history = Vec::with_capacity(BOARD_HISTORY_START_CAPACITY);
        history.push(T::new_board());

        Self {
            board_history: history,
            turn: PieceColor::White,
            constants: Constants::new()
        }
    }

    fn clear_console() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn run(mut game: Game<T>, white_player: Player<T>, black_player: Player<T>) -> Result<EndType, ChessError> {
        let const_ref = Arc::new(game.constants);
        let win_type = loop {
            let opponent_color = game.turn.opposite_color();

            let history_len = game.board_history.len();
            let prev_board = if history_len > 1 { Some(&game.board_history[history_len - 2] ) } else { None };
            let current_board: T = game.board_history[history_len - 1].clone();

            let possible_moves: Vec<(String, T)> = current_board.generate_moves(prev_board, game.turn, &const_ref)?;

            let res: Result<String, ChessError> = match &game.turn {
                PieceColor::White => (white_player.turn_function)(&current_board, prev_board, &game.board_history, game.turn, &white_player, &const_ref),
                PieceColor::Black => (black_player.turn_function)(&current_board, prev_board, &game.board_history, game.turn, &black_player, &const_ref)
            };

            let res: String = match res {
                Ok(res) => {
                    if res.len() < 5 {
                        return Err(ChessError::InvalidMoveString);
                    }
                    res[0..5].to_string()
                },
                Err(err) => {
                    return Err(err);
                }
            };

            if !Self::validate_move_string(&res) {
                return Err(ChessError::InvalidMoveString);
            }

            // for item in &possible_moves {
            //     println!("{}, {}", item.0, item.0 == res);
            // }

            let filtered_moves = possible_moves.into_iter()
                                                       .filter(|mov| (*mov).0 == res)
                                                       .map(|mov| mov.1)
                                                       .collect::<Vec<_>>();

            if filtered_moves.len() == 0 {
                return Err(ChessError::InvalidMove);
            }


            let new_board = &filtered_moves[0];

            println!("{}\n", new_board.board_ascii(true));

            match new_board.check_game_end(Some(&current_board), opponent_color, &const_ref)? {
                EndType::NoEnd => (),
                typ => {
                    game.board_history.push(new_board.clone());
                    break typ;
                }
            }

            // match &new_board.check_repetition(&game.board_history) {
            //     EndType::NoEnd => (),
            //     typ => {
            //         game.board_history.push(new_board);
            //         break typ.clone()
            //     }
            // }

            game.board_history.push(new_board.clone());
            game.turn = opponent_color;
        };

        match win_type {
            EndType::Checkmate(color) => {
                println!("{} won by checkmate!", color.get_string());
                Ok(EndType::Checkmate(color))
            },
            EndType::Tie => {
                println!("Game ended in a tie.");
                Ok(EndType::Tie)
            },
            EndType::NoEnd => Err(ChessError::EndWithNoEnd)
        }
    }

    fn validate_move_string(move_str: &String) -> bool {
        let valid_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let valid_numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];

        let mut characters = move_str.chars();

        if !valid_letters.contains(&mut characters.nth(0).unwrap()) {
            return false;
        }

        if !valid_numbers.contains(&mut characters.nth(0).unwrap()) {
            return false;
        }

        if characters.nth(0).unwrap() != ' ' {
            return false;
        }

        if !valid_letters.contains(&mut characters.nth(0).unwrap()) {
            return false;
        }

        if !valid_numbers.contains(&mut characters.nth(0).unwrap()) {
            return false;
        }

        true
    }
}

