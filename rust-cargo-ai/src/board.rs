#[derive(Debug)]
enum PieceType {
    Pawn = 0,
    Rook = 1,
    Bishop = 2,
    Knight = 3,
    Queen = 4,
    King = 5
}

#[derive(Debug)]
enum PieceColor {
    White = 0,
    Black = 1
}

enum Letter {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

#[derive(Debug)]
struct ChessPiece {
    typ: PieceType,
    color: PieceColor
}

impl ChessPiece {
    fn new(typ: PieceType, color: PieceColor) -> Self {
        Self {
            typ,
            color
        }
    }

    fn piece_to_char(piece: &ChessPiece) -> char {
        match piece.color {
            PieceColor::White => match piece.typ {
                                    PieceType::Pawn => 'p',
                                    PieceType::Rook => 'r',
                                    PieceType::Bishop => 'b',
                                    PieceType::Knight => 'n',
                                    PieceType::Queen => 'q',
                                    PieceType::King => 'k'
                                }
            PieceColor::Black => match piece.typ {
                                    PieceType::Pawn => 'P',
                                    PieceType::Rook => 'R',
                                    PieceType::Bishop => 'B',
                                    PieceType::Knight => 'N',
                                    PieceType::Queen => 'Q',
                                    PieceType::King => 'K'
                                }
        }
    }
}

// Letter is first index then number
pub struct ChessBoard {
    board: [[Option<ChessPiece>; 8]; 8]
}

impl ChessBoard {
    fn blank_board() -> [[Option<ChessPiece>; 8]; 8] {
        [[None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None],
         [None, None, None, None, None, None, None, None]]
    }

    pub fn new_empty_board() -> Self {
        Self {
            board: Self::blank_board()
        }
    }

    pub fn new_start_board() -> Self {
        let mut board = Self::blank_board();

        board[0][0] = Some(ChessPiece {
            typ: PieceType::Rook,
            color: PieceColor::White
        });

        board[1][0] = Some(ChessPiece {
            typ: PieceType::Bishop,
            color: PieceColor::White
        });

        board[2][0] = Some(ChessPiece {
            typ: PieceType::Knight,
            color: PieceColor::White
        });

        board[3][0] = Some(ChessPiece {
            typ: PieceType::Queen,
            color: PieceColor::White
        });

        board[4][0] = Some(ChessPiece {
            typ: PieceType::King,
            color: PieceColor::White
        });

        board[5][0] = Some(ChessPiece {
            typ: PieceType::Knight,
            color: PieceColor::White
        });

        board[6][0] = Some(ChessPiece {
            typ: PieceType::Bishop,
            color: PieceColor::White
        });

        board[7][0] = Some(ChessPiece {
            typ: PieceType::Rook,
            color: PieceColor::White
        });

        for i in 0..8 {
            board[i][1] = Some(ChessPiece {
                typ: PieceType::Pawn,
                color: PieceColor::White
            });
        }

        for i in 0..8 {
            board[i][6] = Some(ChessPiece {
                typ: PieceType::Pawn,
                color: PieceColor::Black
            });
        }

        board[0][7] = Some(ChessPiece {
            typ: PieceType::Rook,
            color: PieceColor::Black
        });

        board[1][7] = Some(ChessPiece {
            typ: PieceType::Bishop,
            color: PieceColor::Black
        });

        board[2][7] = Some(ChessPiece {
            typ: PieceType::Knight,
            color: PieceColor::Black
        });

        board[3][7] = Some(ChessPiece {
            typ: PieceType::Queen,
            color: PieceColor::Black
        });

        board[4][7] = Some(ChessPiece {
            typ: PieceType::King,
            color: PieceColor::Black
        });

        board[5][7] = Some(ChessPiece {
            typ: PieceType::Knight,
            color: PieceColor::Black
        });

        board[6][7] = Some(ChessPiece {
            typ: PieceType::Bishop,
            color: PieceColor::Black
        });

        board[7][7] = Some(ChessPiece {
            typ: PieceType::Rook,
            color: PieceColor::Black
        });

        Self {
            board: board
        }
    }

    pub fn board_ascii(board: &ChessBoard) -> String {
        let mut string = String::with_capacity(577);
        
        for i in (0..8).rev() {
            string.push_str("+---+---+---+---+---+---+---+---+\n");
            for j in 0..8 {
                string.push_str("| ");
                string.push(match &board.board[j][i] {
                    Some(piece) => ChessPiece::piece_to_char(&piece),
                    None => ' '
                });
                string.push(' ');
            }
            string.push_str("|\n");
        }
        string.push_str("+---+---+---+---+---+---+---+---+");

        string
    }
}
