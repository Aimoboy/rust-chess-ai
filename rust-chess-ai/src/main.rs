mod board;
mod game;
mod tests;
mod turn_functions;

fn main() {
    let new_game = game::Game::new();
    game::Game::run(new_game, turn_functions::alpha_beta_pruning_ai, turn_functions::alpha_beta_pruning_ai);
}

fn test_print(node: &turn_functions::Node) {
    println!("{}", node.data.board_ascii(true));

    for child in &node.children {
        test_print(child);
    }
}
