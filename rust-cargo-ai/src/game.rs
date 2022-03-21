use super::board::*;
use std::io::{self, Write};

const BOARD_HISTORY_START_CAPACITY: usize = 100;

pub struct Game {
    board_history: Vec<ChessBoard>,
    turn: PieceColor
}

impl Game {
    pub fn new() -> Self {
        let mut vector = Vec::with_capacity(BOARD_HISTORY_START_CAPACITY);
        vector.push(ChessBoard::new_start_board());

        Self {
            board_history: vector,
            turn: PieceColor::White
        }
    }

    pub fn run(game: Game, white_turn_choice: fn (board: &ChessBoard, turn: &PieceColor) -> String, black_turn_choice: fn (board: &ChessBoard, turn: &PieceColor) -> String) {
        loop {
            let history_len = game.board_history.len();
            let prev_board = if history_len > 1 { Some(&game.board_history[history_len - 2] ) } else { None };
            let current_board = &game.board_history[history_len - 1];
            let move_board = ChessBoard::generate_moveset_board(current_board, prev_board, &game.turn);

            let res: String = match &game.turn {
                PieceColor::White => white_turn_choice(current_board, &game.turn),
                PieceColor::Black => black_turn_choice(current_board, &game.turn)
            };

            if !Self::validate_move_string(&res) {
                println!("Not valid string");
                continue;
            }

            if !Self::validate_move(Self::string_to_move(&res), &move_board) {
                println!("Not valid move");
                continue;
            }

        }
    }

    fn validate_move_string(move_str: &String) -> bool {
        let valid_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let valid_numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];

        let mut characters = move_str.chars();

        if move_str.len() != 7 {
            return false;
        }

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

    fn char_to_index(chr: char, arr: &[char; 8]) -> usize {
        for i in 0..8 {
            if chr == arr[i] {
                return i;
            }
        }

        return usize::MAX;
    }

    fn string_to_move(move_str: &String) -> (Pos, Pos) {
        let valid_letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let valid_numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];

        let mut characters = move_str.chars();

        let letter_from_char = characters.nth(0).unwrap();
        let number_from_char = characters.nth(0).unwrap();

        let letter_to_char = characters.nth(1).unwrap();
        let number_to_char = characters.nth(0).unwrap();

        ((Self::char_to_index(letter_from_char, &valid_letters), Self::char_to_index(number_from_char, &valid_numbers)), (Self::char_to_index(letter_to_char, &valid_letters), Self::char_to_index(number_to_char, &valid_numbers)))
    }

    fn validate_move(m: (Pos, Pos), move_board: &[[Vec<Move>; 8]; 8]) -> bool {
        let ((letter_from, number_from), (letter_to, number_to)) = m;

        let vector = &move_board[letter_from][number_from];

        for i in 0..vector.len() {
            for j in 0..vector[i].moves.len() {
                let (_, to) = vector[i].moves[j];
                if letter_to == to.0 && number_to == to.1 {
                    println!("Validated!");
                    return true;
                }
            }
        }

        false
    }

    pub fn player_move(board: &ChessBoard, turn: &PieceColor) -> String {
        // print!("\x1B[2J\x1B[1;1H");

        let color_str = match turn {
            PieceColor::White => "White",
            PieceColor::Black => "Black"
        };

        println!("It is {}'s turn! (You)", color_str);
        println!("{}", ChessBoard::board_ascii(board));
        print!("\nEnter your move: ");
        std::io::stdout().flush();

        let mut inp = String::new();
        io::stdin().read_line(&mut inp);
        
        inp
    }
}
