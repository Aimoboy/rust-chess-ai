pub type Pos = (usize, usize);
type ReachBoard = [[bool; 8]; 8];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White = 0,
    Black = 1
}

#[derive(PartialEq, Clone, Copy)]
pub enum EndType {
    NoEnd = 0,
    Tie = 1,
    Checkmate = 2
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// enum Letter {
//     A = 0,
//     B = 1,
//     C = 2,
//     D = 3,
//     E = 4,
//     F = 5,
//     G = 6,
//     H = 7
// }

pub enum GetPieceError {
    OutsideBounds = 0,
    NoKing = 1
}

#[derive(Debug, Clone)]
pub struct Move {
    pub moves: Vec<(Pos, Pos)>,
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

#[derive(Debug, Clone)]
pub struct ChessPiece {
    pub typ: PieceType,
    pub color: PieceColor,
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
                                    PieceType::Knight => 'n',
                                    PieceType::Bishop => 'b',
                                    PieceType::Queen => 'q',
                                    PieceType::King => 'k'
                                }
            PieceColor::Black => match piece.typ {
                                    PieceType::Pawn => 'P',
                                    PieceType::Rook => 'R',
                                    PieceType::Knight => 'N',
                                    PieceType::Bishop => 'B',
                                    PieceType::Queen => 'Q',
                                    PieceType::King => 'K'
                                }
        }
    }

    fn piece_to_unicode(piece: &ChessPiece) -> char {
        match piece.color {
            PieceColor::White => match piece.typ {
                                    PieceType::Pawn => '\u{265F}',
                                    PieceType::Rook => '\u{265C}',
                                    PieceType::Knight => '\u{265E}',
                                    PieceType::Bishop => '\u{265D}',
                                    PieceType::Queen => '\u{265B}',
                                    PieceType::King => '\u{265A}'
                                }
            PieceColor::Black => match piece.typ {
                                    PieceType::Pawn => '\u{2659}',
                                    PieceType::Rook => '\u{2656}',
                                    PieceType::Knight => '\u{2658}',
                                    PieceType::Bishop => '\u{2657}',
                                    PieceType::Queen => '\u{2655}',
                                    PieceType::King => '\u{2654}'
                                }
        }
    }
}

// Letter is first index then number
#[derive(Debug, Clone)]
pub struct ChessBoard {
    board: [[Option<ChessPiece>; 8]; 8]
}

impl ChessBoard {
    pub fn get_piece(&self, letter: i32, number: i32) -> Result<Option<&ChessPiece>, GetPieceError> {
        if letter < 0 || letter >= 8 || number < 0 || number >= 8 {
            return Err(GetPieceError::OutsideBounds);
        }

        let safe_letter = letter as usize;
        let safe_number = number as usize;

        Ok(self.board[safe_letter][safe_number].as_ref())
    }

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

        board[1][0] = Some(ChessPiece::new(PieceType::Knight, PieceColor::White));

        board[2][0] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::White));

        board[3][0] = Some(ChessPiece::new(PieceType::Queen, PieceColor::White));

        board[4][0] = Some(ChessPiece::new(PieceType::King, PieceColor::White));

        board[5][0] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::White));

        board[6][0] = Some(ChessPiece::new(PieceType::Knight, PieceColor::White));

        board[7][0] = Some(ChessPiece::new(PieceType::Rook, PieceColor::White));

        for i in 0..8 {
            board[i][1] = Some(ChessPiece::new(PieceType::Pawn, PieceColor::White));
        }

        for i in 0..8 {
            board[i][6] = Some(ChessPiece::new(PieceType::Pawn, PieceColor::Black));
        }

        board[0][7] = Some(ChessPiece::new(PieceType::Rook, PieceColor::Black));

        board[1][7] = Some(ChessPiece::new(PieceType::Knight, PieceColor::Black));

        board[2][7] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black));

        board[3][7] = Some(ChessPiece::new(PieceType::Queen, PieceColor::Black));

        board[4][7] = Some(ChessPiece::new(PieceType::King, PieceColor::Black));

        board[5][7] = Some(ChessPiece::new(PieceType::Bishop, PieceColor::Black));

        board[6][7] = Some(ChessPiece::new(PieceType::Knight, PieceColor::Black));

        board[7][7] = Some(ChessPiece::new(PieceType::Rook, PieceColor::Black));

        Self {
            board: board
        }
    }

    pub fn board_ascii(&self, use_unicode: bool) -> String {
        let mut string = if use_unicode {
            String::with_capacity(844)
        } else {
            String::with_capacity(645)
        };
        
        for i in (0..8).rev() {
            if use_unicode {
                string.push_str("  +----+----+----+----+----+----+----+----+\n");
            } else {
                string.push_str("  +---+---+---+---+---+---+---+---+\n");
            }
            if let Some(res) = std::char::from_digit(1 + i as u32, 10) {
                string.push(res);
                string.push(' ');
            }
            for j in 0..8 {
                string.push_str("| ");
                string.push(match &self.board[j][i] {
                    Some(piece) => if use_unicode {
                        ChessPiece::piece_to_unicode(&piece)
                    } else {
                        ChessPiece::piece_to_char(&piece)
                    },
                    None => ' '
                });
                string.push(' ');

                match &self.board[j][i] {
                    Some(piece) => {
                        if piece.typ != PieceType::Pawn || piece.color != PieceColor::White {
                            string.push(' ');
                        }
                    },
                    None => string.push(' ')
                }
            }
            string.push_str("|\n");
        }
        if use_unicode {
            string.push_str("  +----+----+----+----+----+----+----+----+\n");
        } else {
            string.push_str("  +---+---+---+---+---+---+---+---+\n");
        }

        if use_unicode {
            string.push_str("    A    B    C    D    E    F    G    H");
        } else {
            string.push_str("    A   B   C   D   E   F   G   H");
        }

        string
    }

    pub fn generate_reachable_tiles_board(&self, color: PieceColor) -> ReachBoard {
        let mut reach_board = [[false; 8]; 8];

        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = &self.board[i][j] {
                    if piece.color == color {
                        match piece.typ {
                            PieceType::Pawn => Self::generate_pawn_reach(self, piece, &mut reach_board, (i, j)),
                            PieceType::Rook => Self::generate_rook_reach(self, &mut reach_board, (i, j)),
                            PieceType::Bishop => Self::generate_bishop_reach(self, &mut reach_board, (i, j)),
                            PieceType::Knight => Self::generate_knight_reach(self, &mut reach_board, (i, j)),
                            PieceType::Queen => Self::generate_queen_reach(self, &mut reach_board, (i, j)),
                            PieceType::King => Self::generate_king_reach(self, &mut reach_board, (i, j))
                        };
                    }
                }
            }
        }

        reach_board
    }

    fn generate_pawn_reach(board: &ChessBoard, piece: &ChessPiece, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;
        let side_const = match piece.color {
            PieceColor::White => 1,
            PieceColor::Black => -1
        };

        // Left attack
        if let Ok(_) = board.get_piece(letter as i32 + side_const, number as i32 - 1) {
            reach_board[(letter as i32 + side_const) as usize][number - 1] = true;
        }

        // Right attack
        if let Ok(_) = board.get_piece(letter as i32 + side_const, number as i32 + 1) {
            reach_board[(letter as i32 + side_const) as usize][number + 1] = true;
        }
    }

    fn generate_rook_reach(board: &ChessBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Up, right, down, left
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;
                match board.get_piece(new_letter, new_number) {
                    Ok(tile) => {
                        reach_board[new_letter as usize][new_number as usize] = true;
                        if let Some(_) = tile {
                            break;
                        }
                    },
                    Err(_) => break
                }
            }
        }
    }

    fn generate_bishop_reach(board: &ChessBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Up-right, down-left, down-right, up-left
        let directions = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;
                match board.get_piece(new_letter, new_number) {
                    Ok(tile) => {
                        reach_board[new_letter as usize][new_number as usize] = true;
                        if let Some(_) = tile {
                            break;
                        }
                    },
                    Err(_) => break
                }
            }
        }
    }

    fn generate_knight_reach(board: &ChessBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Clockwise starting from up-right
        let directions = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

        for dir in directions {
        let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;
            if let Ok(_) = board.get_piece(new_letter, new_number) {
                reach_board[new_letter as usize][new_number as usize] = true;
            }
        }
    }

    fn generate_queen_reach(board: &ChessBoard, reach_board: &mut ReachBoard, pos: Pos) {
        Self::generate_rook_reach(board, reach_board, pos);
        Self::generate_bishop_reach(board, reach_board, pos);
    }

    fn generate_king_reach(board: &ChessBoard, reach_board: &mut ReachBoard, pos: Pos) {
        let (letter, number) = pos;

        // Clockwise starting from up-right
        let directions = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];

        for dir in directions {
        let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;
            if let Ok(_) = board.get_piece(new_letter, new_number) {
                reach_board[new_letter as usize][new_number as usize] = true;
            }
        }
    }

    pub fn do_move(&self, mov: &Move) -> ChessBoard {
        let mut new_board = self.clone();

        let moves = &mov.moves;
        let deletion = &mov.deletion;

        if let Some((letter, number)) = deletion {
            new_board.board[*letter][*number] = None;
        }

        for mov in moves {
            let ((letter_from, number_from), (letter_to, number_to)) = mov;
            if let Some(mut new_piece) = new_board.board[*letter_from][*number_from].clone() {
                new_piece.moved = true;

                if new_piece.typ == PieceType::Pawn && (new_piece.color == PieceColor::White && *number_to == 7 || new_piece.color == PieceColor::Black && *number_to == 0) {
                    new_piece.typ = PieceType::Queen;
                }

                new_board.board[*letter_to][*number_to] = Some(new_piece);
                new_board.board[*letter_from][*number_from] = None;
            }
        }

        new_board
    }

    pub fn generate_moveset_board(board: &ChessBoard, previous_board: Option<&ChessBoard>, turn: PieceColor) -> [[Vec<Move>; 8]; 8] {
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
                if let Some(piece) = &board.board[i][j] {
                    if piece.color == turn {
                        Self::generate_moveset(board, previous_board, piece, &mut move_board[i][j], (i, j));
                    }
                }
            }
        }

        move_board
    }

    pub fn get_king_pos(&self, color: PieceColor) -> Result<Pos, GetPieceError> {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = &self.board[i][j] {
                    if piece.typ == PieceType::King && piece.color == color {
                        return Ok((i, j));
                    }
                }
            }
        }

        Err(GetPieceError::NoKing)
    }

    fn check_move(&self, mov: &Move, color: PieceColor) -> bool {
        let new_board = self.do_move(mov);
        let opponent_reach_board = match color {
            PieceColor::White => new_board.generate_reachable_tiles_board(PieceColor::Black),
            PieceColor::Black => new_board.generate_reachable_tiles_board(PieceColor::White)
        };

        if let Ok((king_pos_letter, king_pos_number)) = new_board.get_king_pos(color) {
            return !opponent_reach_board[king_pos_letter][king_pos_number];
        }

        false
    }

    fn generate_moveset(board: &ChessBoard, previous_board: Option<&ChessBoard>, piece: &ChessPiece, move_vector: &mut Vec<Move>, pos: Pos) {
        let possible_moves = match piece.typ {
            PieceType::Pawn => {
                Self::generate_pawn_moves(board, previous_board, piece, pos)
            },
            PieceType::Rook => {
                Self::generate_rook_moves(board, piece, pos)
            },
            PieceType::Knight => {
                Self::generate_knight_moves(board, piece, pos)
            },
            PieceType::Bishop => {
                Self::generate_bishop_moves(board, piece, pos)
            },
            PieceType::Queen => {
                Self::generate_queen_moves(board, piece, pos)
            },
            PieceType::King => {
                Self::generate_king_moves(board, piece, pos)
            }
        };

        for mov in possible_moves {
            if board.check_move(&mov, piece.color) {
                move_vector.push(mov);
            }
        }
    }

    fn generate_pawn_moves(board: &ChessBoard, previous_board: Option<&ChessBoard>, piece: &ChessPiece, pos: Pos) -> Vec<Move> {
        let (letter, number) = pos;
        let mut move_vector = Vec::new();
        let side_const = if piece.color == PieceColor::White { 1 } else { -1 };

        // One step
        let new_number = number as i32 + side_const;
        if let Ok(None) = &board.get_piece(letter as i32, new_number) {
            let moves = vec!((pos, (letter, new_number as usize)));
            let deletion = None;
            move_vector.push(Move::new(moves, deletion));
        }

        // Double step
        if !piece.moved {
            let new_number = number as i32 + side_const;
            if let Ok(None) = &board.get_piece(letter as i32, new_number) {
                let new_number = number as i32 + side_const * 2;
                if let Ok(None) = &board.get_piece(letter as i32, new_number) {
                    let moves = vec!((pos, (letter, new_number as usize)));
                    let deletion = None;
                    move_vector.push(Move::new(moves, deletion));
                }
            }
        }

        // Left, right
        let direction = [-1, 1];

        // Diagonal attack
        for dir in direction {
            let new_letter = letter as i32 + dir;
            let new_number = number as i32 + side_const;
            if let Ok(Some(other_piece)) = board.get_piece(new_letter, new_number) {
                if piece.color != other_piece.color {
                    let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                    let deletion = Some((new_letter as usize, new_number as usize));
                    move_vector.push(Move::new(moves, deletion));
                }
            }
        }

        // En passant
        for dir in direction {
            let new_letter = letter as i32 + dir;
            let new_number = number as i32;
            if let Ok(Some(other_piece)) = board.get_piece(new_letter, new_number) {
                if other_piece.color != piece.color && other_piece.typ == PieceType::Pawn {
                    let new_letter = letter as i32 + dir;
                    let new_number = number as i32 + side_const;
                    if let Ok(None) = board.get_piece(new_letter, new_number) {
                        let new_letter = letter as i32 + dir;
                        let new_number = number as i32 + side_const * 2;
                        if let Some(prev_board) = previous_board {
                            if let Ok(Some(prev_piece)) = prev_board.get_piece(new_letter, new_number) {
                                if prev_piece.color != piece.color && prev_piece.typ == PieceType::Pawn {
                                    let moves = vec!((pos, ((letter as i32 + dir) as usize, (number as i32 + side_const) as usize)));
                                    let deletion = Some(((letter as i32 + dir) as usize, number));
                                    move_vector.push(Move::new(moves, deletion));
                                }
                            }
                        }
                    }
                }
            }
        }

        move_vector
    }

    fn generate_rook_moves(board: &ChessBoard, piece: &ChessPiece, pos: Pos) -> Vec<Move> {
        let (letter, number) = pos;
        let mut move_vector = Vec::new();

        // Up, right, down, left
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;
                match board.get_piece(new_letter, new_number) {
                    Ok(tile) => {
                        match tile {
                            Some(other_piece) => {
                                if other_piece.color != piece.color {
                                    let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                                    let deletion = Some((new_letter as usize, new_number as usize));
                                    move_vector.push(Move::new(moves, deletion));
                                }
                                break;
                            },
                            None => {
                                let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                                let deletion = None;
                                move_vector.push(Move::new(moves, deletion));
                            }
                        }
                    },
                    Err(_) => break
                }
            }
        }

        move_vector
    }

    fn generate_knight_moves(board: &ChessBoard, piece: &ChessPiece, pos: Pos) -> Vec<Move>{
        let (letter, number) = pos;
        let mut move_vector = Vec::new();

        // Clockwise starting from up-right
        let directions = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

        for dir in directions {
        let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;
            if let Ok(tile) = board.get_piece(new_letter, new_number) {
                match tile {
                    Some(other_piece) => {
                        if other_piece.color != piece.color {
                            let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                            let deletion = Some((new_letter as usize, new_number as usize));
                            move_vector.push(Move::new(moves, deletion));
                        }
                    },
                    None => {
                        let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                            let deletion = None;
                            move_vector.push(Move::new(moves, deletion));
                    }
                }
            }
        }

        move_vector
    }

    fn generate_bishop_moves(board: &ChessBoard, piece: &ChessPiece, pos: Pos) -> Vec<Move> {
        let (letter, number) = pos;
        let mut move_vector = Vec::new();

        // Up-right, down-left, down-right, up-left
        let directions = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

        for dir in directions {
            let (letter_const, number_const) = dir;
            for i in 1..8 {
                let new_letter = letter as i32 + i * letter_const;
                let new_number = number as i32 + i * number_const;
                match board.get_piece(new_letter, new_number) {
                    Ok(tile) => {
                        match tile {
                            Some(other_piece) => {
                                if other_piece.color != piece.color {
                                    let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                                    let deletion = Some((new_letter as usize, new_number as usize));
                                    move_vector.push(Move::new(moves, deletion));
                                }
                                break;
                            },
                            None => {
                                let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                                let deletion = None;
                                move_vector.push(Move::new(moves, deletion));
                            }
                        }
                    },
                    Err(_) => break
                }
            }
        }

        move_vector
    }

    fn generate_queen_moves(board: &ChessBoard, piece: &ChessPiece, pos: Pos) -> Vec<Move> {
        let mut move_vector = Vec::new();

        move_vector.append(&mut Self::generate_rook_moves(board, piece, pos));
        move_vector.append(&mut Self::generate_bishop_moves(board, piece, pos));

        move_vector
    }

    fn generate_king_moves(board: &ChessBoard, piece: &ChessPiece, pos: Pos) -> Vec<Move> {
        let (letter, number) = pos;
        let mut move_vector = Vec::new();

        // Clockwise starting from up-right
        let directions = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];

        for dir in directions {
        let (letter_const, number_const) = dir;
            let new_letter = letter as i32 + letter_const;
            let new_number = number as i32 + number_const;
            if let Ok(tile) = board.get_piece(new_letter, new_number) {
                match tile {
                    Some(other_piece) => {
                        if other_piece.color != piece.color {
                            let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                            let deletion = Some((new_letter as usize, new_number as usize));
                            move_vector.push(Move::new(moves, deletion));
                        }
                    },
                    None => {
                        let moves = vec!((pos, (new_letter as usize, new_number as usize)));
                            let deletion = None;
                            move_vector.push(Move::new(moves, deletion));
                    }
                }
            }
        }

        let opponent_reach_board = match piece.color {
            PieceColor::White => board.generate_reachable_tiles_board(PieceColor::Black),
            PieceColor::Black => board.generate_reachable_tiles_board(PieceColor::White)
        }; 

        // Left castle
        if let Ok(Some(king_piece)) = board.get_piece(4, number as i32) {
            if king_piece.typ == PieceType::King && !king_piece.moved {
                if let Ok(Some(rook_piece)) = board.get_piece(0, number as i32) {
                    if rook_piece.typ == PieceType::Rook && !rook_piece.moved {
                        if let Ok(None) = board.get_piece(1, number as i32) {
                            if let Ok(None) = board.get_piece(2, number as i32) {
                                if let Ok(None) = board.get_piece(3, number as i32) {
                                    // Check if tiles are threatened
                                    if !opponent_reach_board[2][number] && !opponent_reach_board[3][number] && !opponent_reach_board[4][number] {
                                        let moves = vec![(pos, (2, number)), ((0, number), (3, number))];
                                        let deletion = None;
                                        move_vector.push(Move::new(moves, deletion));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Right castle
        if let Ok(Some(king_piece)) = board.get_piece(4, number as i32) {
            if king_piece.typ == PieceType::King && !king_piece.moved {
                if let Ok(Some(rook_piece)) = board.get_piece(7, number as i32) {
                    if rook_piece.typ == PieceType::Rook && !rook_piece.moved {
                        if let Ok(None) = board.get_piece(5, number as i32) {
                            if let Ok(None) = board.get_piece(6, number as i32) {
                                // Check if tiles are threatened
                                if !opponent_reach_board[4][number] && !opponent_reach_board[5][number] && !opponent_reach_board[6][number] {
                                    let moves = vec![(pos, (6, number)), ((7, number), (5, number))];
                                    let deletion = None;
                                    move_vector.push(Move::new(moves, deletion));
                                }
                            }
                        }
                    }
                }
            }
        }

        move_vector
    }

    // Check if the game has ended for the given player
    pub fn check_for_game_end(&self, prev_board: Option<&ChessBoard>, turn: PieceColor) -> EndType {
        let opposite_color = match turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        };

        let move_board = ChessBoard::generate_moveset_board(self, prev_board, turn);

        let mut move_count: usize = 0;

        for i in 0..8 {
            for j in 0..8 {
                move_count += move_board[i][j].len();
            }
        }

        if move_count == 0 {
            let reach_board = self.generate_reachable_tiles_board(opposite_color);

            if let Ok((letter, number)) = self.get_king_pos(turn) {
                if reach_board[letter][number] {
                    return EndType::Checkmate;
                } else {
                    return EndType::Tie;
                }
            }
        }

        return EndType::NoEnd;
    }
}
