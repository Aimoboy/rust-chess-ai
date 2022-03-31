mod board;
mod bitboard;
mod game;
mod tests;
mod turn_functions;
mod piece_color;

use std::collections::HashMap;

// player_move
// alpha_beta_pruning_ai
// alpha_beta_pruning_ai_tree_generate_with_threads


fn main() {
    // println!("{:?}", bitboard::generate_row_and_column_mask());
    // println!("{:?}", bitboard::pos_to_num(2, 4));
    // println!("{:?}", bitboard::num_to_pos(34));
    // bitboard::print_bitboard(72340172838076926);
    // let new_game = game::Game::new();
    // game::Game::run(new_game, turn_functions::player_move, turn_functions::alpha_beta_pruning_ai_tree_generate_with_threads, 0, 5);
    // let x = [0 as u64; 1048576];
    // let x: HashMap<u64, u64> = HashMap::with_capacity(1048576);
    // bitboard::test();

    let now = std::time::Instant::now();
    let c = bitboard::Constants::new();
    println!("{}", now.elapsed().as_millis() as f32 / 1000.);
    let ascii = bitboard::get_bitboard_ascii(&c.start_board);
    println!("{}", ascii);
    println!();
    let mut new_board = c.start_board;
    new_board[5] = 0;
    new_board[5] += 1 << 36;
    let moves = bitboard::generate_possible_moves(&new_board, None, piece_color::PieceColor::White, &c);
    for mov in moves {
        println!("{}", bitboard::get_bitboard_ascii(&mov.1));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    // bitboard::print_bitboard(bitboard::get_reach_board(&c.start_board, piece_color::PieceColor::Black, &c));
}
