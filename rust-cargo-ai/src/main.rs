mod board;

fn main() {
    let b = board::ChessBoard::new_start_board();
    let string = board::ChessBoard::board_ascii(&b);

    println!("{}", string);
    println!("{}", string.len());
    println!("{}", string.capacity());
}
