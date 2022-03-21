mod board;
mod game;

fn main() {
    let b = board::ChessBoard::new_start_board();
    let string = board::ChessBoard::board_ascii(&b);
    println!("{}", string);

    //print!("\x1B[2J\x1B[1;1H");

    let new_game = game::Game::new();
    game::Game::run(new_game, game::Game::player_move, game::Game::player_move);
}
