mod board;
mod game;
mod tests;
mod turn_functions;


// player_move
// alpha_beta_pruning_ai
// alpha_beta_pruning_ai_tree_generate_with_threads


fn main() {
    let new_game = game::Game::new();
    game::Game::run(new_game, turn_functions::alpha_beta_pruning_ai, turn_functions::alpha_beta_pruning_ai_tree_generate_with_threads, 2, 4);
}
