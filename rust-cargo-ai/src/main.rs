mod board;
mod game;

fn main() {
    let new_game = game::Game::new();
    game::Game::run(new_game, game::Game::player_move, game::Game::player_move);
}
