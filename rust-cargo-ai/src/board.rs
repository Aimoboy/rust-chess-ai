type Pos = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Bishop = 2,
    Knight = 3,
    Queen = 4,
    King = 5
}

#[derive(Debug, PartialEq)]
pub enum PieceColor {
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

pub struct Move {
    moves: Vec<(Pos, Pos)>,
    deletion: Option<Pos>
}

impl Move {
    fn new(moves: Vec<(Pos, Pos)>, deletion: Option<Pos>) -> Self{
        Self {
            moves,
            deletion
        }
    }
}

#[derive(Debug)]
struct ChessPiece {
    typ: PieceType,
    color: PieceColor,
    moved: bool
}

impl ChessPiece {
    fn new(typ: PieceType, color: PieceColor) -> Self {
        Self {
            typ,
            color,
            moved: false
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

        board[0][0] = Some(ChessPiece::new(PieceType::Rook, PieceColor::White));

        board[1][0] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::White));

        board[2][0] = Some(ChessPiece::new(PieceType::Knight, PieceColor::White));

        board[3][0] = Some(ChessPiece::new(PieceType::Queen, PieceColor::White));

        board[4][0] = Some(ChessPiece::new(PieceType::King, PieceColor::White));

        board[5][0] = Some(ChessPiece::new(PieceType::Knight, PieceColor::White));

        board[6][0] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::White));

        board[7][0] = Some(ChessPiece::new(PieceType::Rook, PieceColor::White));

        for i in 0..8 {
            board[i][1] = Some(ChessPiece::new(PieceType::Pawn, PieceColor::White));
        }

        for i in 0..8 {
            board[i][6] = Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black));
        }

        board[0][7] = Some(ChessPiece::new(PieceType::Rook, PieceColor::Black));

        board[1][7] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black));

        board[2][7] = Some(ChessPiece::new(PieceType::Knight, PieceColor::Black));

        board[3][7] = Some(ChessPiece::new(PieceType::Queen, PieceColor::Black));

        board[4][7] = Some(ChessPiece::new(PieceType::King, PieceColor::Black));

        board[5][7] = Some(ChessPiece::new(PieceType::Knight, PieceColor::Black));

        board[6][7] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black));

        board[7][7] = Some(ChessPiece::new(PieceType::Rook, PieceColor::Black));

        Self {
            board: board
        }
    }

    pub fn board_ascii(board: &ChessBoard) -> String {
        let mut string = String::with_capacity(645);
        
        for i in (0..8).rev() {
            string.push_str("  +---+---+---+---+---+---+---+---+\n");
            if let Some(res) = std::char::from_digit(1 + i as u32, 10) {
                string.push(res);
                string.push(' ');
            }
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
        string.push_str("  +---+---+---+---+---+---+---+---+\n");
        string.push_str("    A   B   C   D   E   F   G   H");

        string
    }

    pub fn generate_moveset_board(board: &ChessBoard, previous_board: Option<&ChessBoard>, color: PieceColor) -> [[Vec<Move>; 8]; 8] {
        const START_CAPACITY: usize = 15;
        let mut move_board = [[Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)],
                              [Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)],
                              [Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)],
                              [Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)],
                              [Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)],
                              [Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)],
                              [Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)],
                              [Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY), Vec::with_capacity(START_CAPACITY)]];
        
        for i in 0..8 {
            for j in 0..8 {
                match &board.board[i][j] {
                    Some(piece) => {
                        Self::generate_moveset(board, previous_board, piece, &mut move_board[i][j], (i, j));
                    },
                    None => ()
                }
            }
        }

        move_board
    }

    fn generate_moveset(board: &ChessBoard, previous_board: Option<&ChessBoard>, piece: &ChessPiece, move_vector: &mut Vec<Move>, pos: Pos) {
        let (letter, number) = pos;

        match piece.typ {
            PieceType::Pawn => {
                Self::generate_pawn_moves(board, previous_board, piece, move_vector, pos);
            },
            PieceType::Rook => {
                Self::generate_rook_moves(board, piece, move_vector, pos);
            },
            PieceType::Bishop => {
                Self::generate_bishop_moves(board, piece, move_vector, pos);
            },
            PieceType::Knight => {
                Self::generate_knight_moves(board, piece, move_vector, pos);
            },
            PieceType::Queen => {
                Self::generate_rook_moves(board, piece, move_vector, pos);
                Self::generate_bishop_moves(board, piece, move_vector, pos);
            },
            PieceType::King => {

            }
        }
    }

    fn generate_pawn_moves(board: &ChessBoard, previous_board: Option<&ChessBoard>, piece: &ChessPiece, move_vector: &mut Vec<Move>, pos: Pos) {
        let (letter, number) = pos;
        let side = if piece.color == PieceColor::White { 1 } else { -1 };

        // One step
        let number_new = number as i32 + side;
        if 0 <= number_new && number_new < 8 {
            let number_new = number_new as usize;
            if board.board[letter][number_new].is_none() {
                let moves = vec!((pos, (letter, number_new)));
                let deletion = None;
                move_vector.push(Move::new(moves, deletion));
            }
        }

        // Double step
        let number_new = number as i32 + side * 2;
        if !piece.moved {
            let number_new = number_new as usize;
            if board.board[letter][number_new].is_none() {
                let moves = vec!((pos, (letter, number_new)));
                let deletion = None;
                move_vector.push(Move::new(moves, deletion));
            }
        }

        // Left diagonal step
        let (letter_new, number_new) = (letter as i32 - 1 , number as i32 + side);
        if 0 <= letter_new && letter_new < 8 && 0 <= number_new && number_new < 8 {
            let (letter_new, number_new) = (letter_new as usize, number_new as usize);

            if let Some(other_piece) = &board.board[letter_new][number_new] {
                if piece.color != other_piece.color {
                    let moves = vec!((pos, (letter_new, number_new)));
                    let deletion = Some((letter_new, number_new));
                    move_vector.push(Move::new(moves, deletion));
                }
            }
        }

        // Right diagonal step
        let (letter_new, number_new) = (letter as i32 + 1 , number as i32 + side);
        if 0 <= letter_new && letter_new < 8 && 0 <= number_new && number_new < 8 {
            let (letter_new, number_new) = (letter_new as usize, number_new as usize);

            if let Some(other_piece) = &board.board[letter_new][number_new] {
                if piece.color != other_piece.color {
                    let moves = vec!((pos, (letter_new, number_new)));
                    let deletion = Some((letter_new, number_new));
                    move_vector.push(Move::new(moves, deletion));
                }
            }
        }

        // Left en passant
        if letter > 0 && 0 <= number as i32 + side * 2 && number as i32 + side * 2 < 8 {
            let number_plus_one = (number as i32 + side) as usize;
            let number_plus_two = (number as i32 + side * 2) as usize;
            if let Some(other_piece) = &board.board[letter - 1][number] {
                if piece.color != other_piece.color && other_piece.typ == PieceType::Pawn {
                    if board.board[letter - 1][number_plus_one].is_none() {
                        if let Some(previous_board) = previous_board {
                            if let Some(prev_piece) = &previous_board.board[letter - 1][number_plus_two] {
                                if piece.color != prev_piece.color && prev_piece.typ == PieceType::Pawn && !prev_piece.moved {
                                    let moves = vec!((pos, (letter - 1, number + number_plus_one)));
                                    let deletion = Some((letter - 1, number));
                                    move_vector.push(Move::new(moves, deletion));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Right en passant
        if letter < 7 && 0 <= number as i32 + side * 2 && number as i32 + side * 2 < 8 {
            let number_plus_one = (number as i32 + side) as usize;
            let number_plus_two = (number as i32 + side * 2) as usize;
            if let Some(other_piece) = &board.board[letter + 1][number] {
                if piece.color != other_piece.color && other_piece.typ == PieceType::Pawn {
                    if board.board[letter + 1][number_plus_one].is_none() {
                        if let Some(previous_board) = previous_board {
                            if let Some(prev_piece) = &previous_board.board[letter + 1][number_plus_two] {
                                if piece.color != prev_piece.color && prev_piece.typ == PieceType::Pawn && !prev_piece.moved {
                                    let moves = vec!((pos, (letter + 1, number + number_plus_one)));
                                    let deletion = Some((letter + 1, number));
                                    move_vector.push(Move::new(moves, deletion));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn generate_rook_moves(board: &ChessBoard, piece: &ChessPiece, move_vector: &mut Vec<Move>, pos: Pos) {
        let (letter, number) = pos;

        // Up
        for i in 1..8 {
            if number + i < 8 {
                match &board.board[letter][number + i] {
                    None => {
                        let moves = vec!((pos, (letter, number + i)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter, number + i)));
                            let deletion = Some((letter, number + i));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }

        // Down
        for i in 1..8 {
            if number >= i {
                match &board.board[letter][number - i] {
                    None => {
                        let moves = vec!((pos, (letter, number - i)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter, number - i)));
                            let deletion = Some((letter, number - i));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }

        // Right
        for i in 1..8 {
            if letter + i < 8 {
                match &board.board[letter + i][number] {
                    None => {
                        let moves = vec!((pos, (letter + i, number)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter + i, number)));
                            let deletion = Some((letter + i, number));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }

        // Left
        for i in 1..8 {
            if letter >= i {
                match &board.board[letter - i][number] {
                    None => {
                        let moves = vec!((pos, (letter - i, number)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter - i, number)));
                            let deletion = Some((letter - i, number));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }
    }

    fn generate_bishop_moves(board: &ChessBoard, piece: &ChessPiece, move_vector: &mut Vec<Move>, pos: Pos) {
        let (letter, number) = pos;

        // Top-right
        for i in 1..8 {
            if letter + i < 8 && number + i < 8 {
                match &board.board[letter + i][number + i] {
                    None => {
                        let moves = vec!((pos, (letter + i, number + i)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter + i, number + i)));
                            let deletion = Some((letter + i, number + i));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }

        // Bottom-left
        for i in 1..8 {
            if letter >= i && number >= i {
                match &board.board[letter - i][number - i] {
                    None => {
                        let moves = vec!((pos, (letter - i, number - i)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter - i, number - i)));
                            let deletion = Some((letter - i, number - i));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }

        // Top-left
        for i in 1..8 {
            if letter >= i && number + i < 8 {
                match &board.board[letter - i][number + i] {
                    None => {
                        let moves = vec!((pos, (letter - i, number + i)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter - i, number + i)));
                            let deletion = Some((letter - i, number + i));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }

        // Bottom-right
        for i in 1..8 {
            if letter + i < 8 && number >= i {
                match &board.board[letter + i][number - i] {
                    None => {
                        let moves = vec!((pos, (letter + i, number - i)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    },
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter + i, number - i)));
                            let deletion = Some((letter + i, number - i));
                            move_vector.push(Move::new(moves, deletion));
                            break;
                        }
                    }
                }
            }
        }
    }

    fn generate_knight_moves(board: &ChessBoard, piece: &ChessPiece, move_vector: &mut Vec<Move>, pos: Pos) {
        let (letter, number) = pos;

        let possible_moves = [(-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1)];

        for p in possible_moves {
            let (letter_diff, number_diff) = p;
            let (letter_new, number_new) = (letter as i32 + letter_diff, number as i32 + number_diff);
            if 0 <= letter_new && letter_new < 8 && 0 <= number_new && number_new < 8 {
                let (letter_new, number_new) = (letter_new as usize, number_new as usize);
                match &board.board[letter_new][number_new] {
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter_new, number_new)));
                            let deletion = Some((letter_new, number_new));
                            move_vector.push(Move::new(moves, deletion));
                        }
                    },
                    None => {
                        let moves = vec!((pos, (letter_new, number_new)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    }
                }
            }
        }
    }

    fn generate_king_moves(board: &ChessBoard, piece: &ChessPiece, move_vector: &mut Vec<Move>, pos: Pos) {
        let (letter, number) = pos;

        // Normal king moves
        let possible_moves = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
        
        for p in possible_moves {
            let (letter_diff, number_diff) = p;
            let (letter_new, number_new) = (letter as i32 + letter_diff, number as i32 + number_diff);
            if 0 <= letter_new && letter_new < 8 && 0 <= number_new && number_new < 8 {
                let (letter_new, number_new) = (letter_new as usize, number_new as usize);

                match &board.board[letter_new][number_new] {
                    Some(other_piece) => {
                        if piece.color != other_piece.color {
                            let moves = vec!((pos, (letter_new, number_new)));
                            let deletion = Some((letter_new, number_new));
                            move_vector.push(Move::new(moves, deletion));
                        }
                    },
                    None => {
                        let moves = vec!((pos, (letter_new, number_new)));
                        let deletion = None;
                        move_vector.push(Move::new(moves, deletion));
                    }
                }
            }
        }

        // Left castle
        if letter == 4 && !piece.moved {
            if board.board[3][number].is_none() && board.board[2][number].is_none() && board.board[1][number].is_none() {
                if let Some(rook_piece) = &board.board[0][number] {
                    if piece.color == rook_piece.color && rook_piece.typ == PieceType::Rook && !rook_piece.moved {
                        let moves = vec![(pos, (2, number)), ((0, number), (3, number))];
                            let deletion = None;
                            move_vector.push(Move::new(moves, deletion));
                    }
                }
            }
        }

        // Right castle
        if letter == 4 && !piece.moved {
            if board.board[5][number].is_none() && board.board[6][number].is_none() {
                if let Some(rook_piece) = &board.board[7][number] {
                    if piece.color == rook_piece.color && rook_piece.typ == PieceType::Rook && !rook_piece.moved {
                        let moves = vec![(pos, (6, number)), ((7, number), (5, number))];
                            let deletion = None;
                            move_vector.push(Move::new(moves, deletion));
                    }
                }
            }
        }

    }
}
