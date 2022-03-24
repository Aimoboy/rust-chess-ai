mod board;
mod game;
mod tests;
mod turn_functions;

fn main() {
    let new_game = game::Game::new();
    game::Game::run(new_game, turn_functions::player_move, turn_functions::alpha_beta_pruning_ai);

    // let b = board::ChessBoard::new_start_board();
    // let tree = turn_functions::generate_board_tree(&b, board::PieceColor::White);

    // test_print(&tree);
    // println!("Height: {}", tree.get_height());
    // println!("Node count: {}", tree.count_nodes());
}

fn test_print(node: &turn_functions::Node) {
    println!("{}", node.data.board_ascii(true));

    for child in &node.children {
        test_print(child);
    }
}