
#[cfg(test)]
mod normalboard_unit_tests {
    use crate::board_types::normalboard::NormalBoard;
    use crate::enums::{
        piece_color::PieceColor,
        piece_type::PieceType
    };

    // #[test]
    // fn make_empty_board() {
    //     let board = NormalBoard::new_empty_board();

    //     for i in 0..8 {
    //         for j in 0..8 {
    //             if let Ok(piece) = &board.get_piece(i, j) {
    //                 assert_eq!(None, *piece);
    //             } else {
    //                 assert!(false);
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn set_piece() {
    //     let mut board = NormalBoard::new_empty_board();

    //     let get_piece = board.get_piece(0, 0);
    //     match get_piece {
    //         Ok(None) => (),
    //         _ => assert!(false)
    //     }

    //     let res = board.set_piece(0, 0, PieceType::Pawn, PieceColor::White);

    //     match res {
    //         Ok(true) => (),
    //         _ => assert!(false)
    //     }

    //     let get_piece = board.get_piece(0, 0);
    //     match get_piece {
    //         Ok(piece) => {
    //             assert_ne!(None, piece);

    //             if let Some(piece) = piece {
    //                 assert_eq!(piece.typ, PieceType::Pawn);
    //                 assert_eq!(piece.color, PieceColor::White);
    //             }
    //         },
    //         _ => assert!(false)
    //     }

    // }

    // To do:
    // Make start board
    // Get piece
    // En passent
    // Left castle
    // Right castle
    // Pawn gets turned into piece on the other side of the board
    // Correct reach for each piece
    // Correct possible moves for each piece
    // Draw by repetition
    // Checkmate
    // King cannot move into check
    // Cannot move another piece that would result in king being in check
}
