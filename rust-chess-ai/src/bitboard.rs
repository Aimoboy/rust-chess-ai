use super::piece_color::*;
use std::collections::HashMap;

type BitBoard = [u64; 12];
type BitBoardMove = (((u64, u64), (u64, u64)), BitBoard);

enum PieceNum {
    WhitePawn = 0,
    WhiteRook = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackRook = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackQueen = 10,
    BlackKing = 11
}

pub struct Constants {
    pub start_board: BitBoard,
    pub row_and_column_mask: [u64; 64],
    pub diagonal_mask: [u64; 64],
    pub pawn_reach: [[u64; 64]; 2],
    pub rook_reach: [HashMap<u64, u64>; 64],
    pub knight_reach: [u64; 64],
    pub bishop_reach: [HashMap<u64, u64>; 64],
    pub king_reach: [u64; 64]
}

impl Constants {
    pub fn new() -> Self {
        Self {
            start_board: generate_start_board(),
            row_and_column_mask: generate_row_and_column_mask(),
            diagonal_mask: generate_diagonal_mask(),
            pawn_reach: generate_pawn_reach(),
            rook_reach: generate_rook_reach(),
            knight_reach: generate_knight_reach(),
            bishop_reach: generate_bishop_reach(),
            king_reach: generate_king_reach()
        }
    }
}

fn get_opposite_color(color: PieceColor) -> PieceColor{
    match color {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White
    }
}

fn generate_start_board() -> BitBoard {
    let mut possible_moves = [0; 12];

    // White pawns
    for i in 8..16 {
        possible_moves[0] += 1 << i;
    }

    // White rooks
    possible_moves[1] += 1 << 0;
    possible_moves[1] += 1 << 7;

    // White knights
    possible_moves[2] += 1 << 1;
    possible_moves[2] += 1 << 6;

    // White bishops
    possible_moves[3] += 1 << 2;
    possible_moves[3] += 1 << 5;

    // White queen
    possible_moves[4] += 1 << 3;

    // White king
    possible_moves[5] += 1 << 4;

    // Black pawns
    for i in 48..56 {
        possible_moves[6] += 1 << i;
    }

    // Black rooks
    possible_moves[7] += 1 << 56;
    possible_moves[7] += 1 << 63;

    // Black knights
    possible_moves[8] += 1 << 57;
    possible_moves[8] += 1 << 62;

    // Black bishops
    possible_moves[9] += 1 << 58;
    possible_moves[9] += 1 << 61;

    // Black queen
    possible_moves[10] += 1 << 59;

    // Black king
    possible_moves[11] += 1 << 60;

    possible_moves
}

fn generate_row_and_column_mask() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut tmp = 0;

        // Up
        for j in number + 1..8 {
            tmp += 1 << pos_to_num(letter, j);
        }

        // Down
        for j in 0..number {
            tmp += 1 << pos_to_num(letter, j);
        }

        // Left
        for j in 0..letter {
            tmp += 1 << pos_to_num(j, number);
        }

        // Right
        for j in letter + 1..8 {
            tmp += 1 << pos_to_num(j, number);
        }

        possible_moves[i as usize] = tmp;
    }


    possible_moves
}

fn generate_diagonal_mask() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut tmp = 0;

        // Up-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Down-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Down-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        // Up-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            tmp += 1 << pos_to_num(new_letter as u64, new_number as u64);
        }

        possible_moves[i as usize] = tmp;
    }


    possible_moves
}

fn generate_pawn_reach() -> [[u64; 64]; 2] {
    let mut possible_moves = [[0; 64]; 2];
    let side_constants = [1, -1];

    for side in side_constants {
        for i in 0..64 {
            let (letter, number) = num_to_pos(i);
            let mut b = 0;

            // Right
            let new_letter = letter as i32 + 1;
            let new_number = number as i32 + side;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }

            // Left
            let new_letter = letter as i32 - 1;
            let new_number = number as i32 + side;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }

            if side == 1 {
                possible_moves[0][i as usize] = b;
            } else {
                possible_moves[1][i as usize] = b;
            }
        }
    }

    possible_moves
}

fn generate_rook_reach() -> [HashMap<u64, u64>; 64] {
    let mut possible_moves: [HashMap<u64, u64>; 64] = [(); 64].map(|_| HashMap::with_capacity(16384));

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut possibilities = Vec::with_capacity(16384);
        possibilities.push(0);
        let mut points = Vec::with_capacity(14);

        // Up
        for j in number + 1..8 {
            points.push(pos_to_num(letter, j));
        }

        // Down
        for j in 0..number {
            points.push(pos_to_num(letter, j));
        }

        // Left
        for j in 0..letter {
            points.push(pos_to_num(j, number));
        }

        // Right
        for j in letter + 1..8 {
            points.push(pos_to_num(j, number));
        }

        let mut b = 0;
        let mut k = 0;
        while k < points.len() {
            let num_pos = points[k];
            if b & (1 << num_pos) == 0 {
                b += 1 << num_pos;
                for l in 0..k {
                    let num_pos = points[l];
                    b -= 1 << num_pos;
                }
                possibilities.push(b);
                k = 0;
            } else {
                k += 1;
            }
        }

        for p in possibilities {
            let mut b = 0;

            // Up
            for j in number + 1..8 {
                let pos = 1 << pos_to_num(letter, j);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down
            for j in (0..number).rev() {
                let pos = 1 << pos_to_num(letter, j);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Left
            for j in (0..letter).rev() {
                let pos = 1 << pos_to_num(j, number);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Right
            for j in letter + 1..8 {
                let pos = 1 << pos_to_num(j, number);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            possible_moves[i as usize].insert(p, b);
        }
    }

    possible_moves
}

fn generate_knight_reach() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    // Clockwise starting from up-right
    let position_diffs = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut b = 0;

        for diffs in position_diffs {
            let (letter_diff, number_diff) = diffs;
            let new_letter = letter as i32 + letter_diff;
            let new_number = number as i32 + number_diff;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }
        }

        possible_moves[i as usize] = b;
    }

    possible_moves
}

fn generate_bishop_reach() -> [HashMap<u64, u64>; 64] {
    let mut possible_moves: [HashMap<u64, u64>; 64] = [(); 64].map(|_| HashMap::with_capacity(16384));

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut possibilities = Vec::with_capacity(16384);
        possibilities.push(0);
        let mut points = Vec::with_capacity(14);

        // Up-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Down-right
        for j in 1..7 {
            let new_letter = letter as i32 + j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Down-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 - j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        // Up-left
        for j in 1..7 {
            let new_letter = letter as i32 - j;
            let new_number = number as i32 + j;
            if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                break;
            }
            points.push(pos_to_num(new_letter as u64, new_number as u64));
        }

        let mut b = 0;
        let mut k = 0;
        while k < points.len() {
            let num_pos = points[k];
            if b & (1 << num_pos) == 0 {
                b += 1 << num_pos;
                for l in 0..k {
                    let num_pos = points[l];
                    b -= 1 << num_pos;
                }
                possibilities.push(b);
                k = 0;
            } else {
                k += 1;
            }
        }

        for p in possibilities {
            let mut b = 0;

            // Up-right
            for j in 1..7 {
                let new_letter = letter as i32 + j;
                let new_number = number as i32 + j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down-right
            for j in 1..7 {
                let new_letter = letter as i32 + j;
                let new_number = number as i32 - j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Down-left
            for j in 1..7 {
                let new_letter = letter as i32 - j;
                let new_number = number as i32 - j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            // Up-left
            for j in 1..7 {
                let new_letter = letter as i32 - j;
                let new_number = number as i32 + j;
                if 0 > new_letter || new_letter > 7 || 0 > new_number || new_number > 7 {
                    break;
                }
                let pos = 1 << pos_to_num(new_letter as u64, new_number as u64);
                b += pos;
                if p & pos == pos {
                    break;
                }
            }

            possible_moves[i as usize].insert(p, b);
        }
    }

    possible_moves
}

fn generate_king_reach() -> [u64; 64] {
    let mut possible_moves = [0; 64];

    // Clockwise starting from up-right
    let position_diffs = [(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];

    for i in 0..64 {
        let (letter, number) = num_to_pos(i);
        let mut b = 0;

        for diffs in position_diffs {
            let (letter_diff, number_diff) = diffs;
            let new_letter = letter as i32 + letter_diff;
            let new_number = number as i32 + number_diff;
            if 0 <= new_letter && new_letter < 8 && 0 <= new_number && new_number < 8 {
                b += 1 << pos_to_num(new_letter as u64, new_number as u64);
            }
        }

        possible_moves[i as usize] = b;
    }

    possible_moves
}

pub fn print_bitboard(num: u64) {
    let board = format!("{:#066b}", num).chars().rev().collect::<String>();
    for i in (0..8).rev() {
        println!("{}", board[i*8..i*8+8].to_string());
    }
}

pub fn pos_to_num(letter: u64, number: u64) -> u64 {
    letter + (number << 3)
}

pub fn num_to_pos(num: u64) -> (u64, u64) {
    (num & 7, num >> 3)
}

fn get_full_color_board(board: &BitBoard, color: PieceColor) -> u64 {
    let mut possible_moves: u64 = 0;
    match color {
        PieceColor::White => {
            possible_moves |= board[PieceNum::WhitePawn as usize];
            possible_moves |= board[PieceNum::WhiteRook as usize];
            possible_moves |= board[PieceNum::WhiteKnight as usize];
            possible_moves |= board[PieceNum::WhiteBishop as usize];
            possible_moves |= board[PieceNum::WhiteQueen as usize];
            possible_moves |= board[PieceNum::WhiteKing as usize];
        },
        PieceColor::Black => {
            possible_moves |= board[PieceNum::BlackPawn as usize];
            possible_moves |= board[PieceNum::BlackRook as usize];
            possible_moves |= board[PieceNum::BlackKnight as usize];
            possible_moves |= board[PieceNum::BlackBishop as usize];
            possible_moves |= board[PieceNum::BlackQueen as usize];
            possible_moves |= board[PieceNum::BlackKing as usize];
        }
    }
    possible_moves
}

fn get_occupied_board(board: &BitBoard) -> u64 {
    get_full_color_board(board, PieceColor::White) | get_full_color_board(board, PieceColor::Black)
}

pub fn get_reach_board(board: &BitBoard, color: PieceColor, constants: &Constants) -> u64 {
    let occupied_board = get_occupied_board(board);

    let mut b = 0;

    match color {
        PieceColor::White => {
            for i in 0..64 {
                let num = 1 << i;
                let rows_and_columns = occupied_board & constants.row_and_column_mask[i];
                let diagonals = occupied_board & constants.diagonal_mask[i];

                if board[PieceNum::WhitePawn as usize] & num == num {
                    b |= constants.pawn_reach[0][i];
                } else if board[PieceNum::WhiteRook as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                } else if board[PieceNum::WhiteKnight as usize] & num == num {
                    b |= constants.knight_reach[i];
                } else if board[PieceNum::WhiteBishop as usize] & num == num {
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::WhiteQueen as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::WhiteKing as usize] & num == num {
                    b |= constants.king_reach[i];
                }
            }
        },
        PieceColor::Black => {
            for i in 0..64 {
                let num = 1 << i;
                let rows_and_columns = occupied_board & constants.row_and_column_mask[i];
                let diagonals = occupied_board & constants.diagonal_mask[i];

                if board[PieceNum::BlackPawn as usize] & num == num {
                    b |= constants.pawn_reach[1][i];
                } else if board[PieceNum::BlackRook as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                } else if board[PieceNum::BlackKnight as usize] & num == num {
                    b |= constants.knight_reach[i];
                } else if board[PieceNum::BlackBishop as usize] & num == num {
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::BlackQueen as usize] & num == num {
                    b |= constants.rook_reach[i].get(&rows_and_columns).unwrap();
                    b |= constants.bishop_reach[i].get(&diagonals).unwrap();
                } else if board[PieceNum::BlackKing as usize] & num == num {
                    b |= constants.king_reach[i];
                }
            }
        }
    }

    b
}

fn is_in_check(board: &BitBoard, color: PieceColor, constants: &Constants) -> bool {
    let opposite_reach_board = get_reach_board(&board, get_opposite_color(color), &constants);
    match color {
        PieceColor::White => {
            let king_num = board[PieceNum::WhiteKing as usize];
            opposite_reach_board & king_num == king_num
        },
        PieceColor::Black => {
            let king_num = board[PieceNum::BlackKing as usize];
            opposite_reach_board & king_num == king_num
        }
    }
}

pub fn generate_possible_moves(board: &BitBoard, prev_board: Option<&BitBoard>, color: PieceColor, constants: &Constants) -> Vec<BitBoardMove> {
    let mut possible_moves = Vec::new();

    let opposite_color = get_opposite_color(color);
    let occupied_board = get_occupied_board(&board);
    let own_pieces = get_full_color_board(&board, color);
    let opposite_pieces = get_full_color_board(&board, opposite_color);
    let reach_board = get_reach_board(&board, color, &constants);
    let opposite_reach_board = get_reach_board(&board, opposite_color, &constants);

    match color {
        PieceColor::White => {
            // Pawns
            let mut tmp = board[PieceNum::WhitePawn as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                if occupied_board & 1 << (i + 8) == 0 {
                    let mut new_board = board.clone();
                    new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                    if 55 <= i + 8 && i + 8 < 64 {
                        new_board[PieceNum::WhiteQueen as usize] += 1 << (i + 8);
                    } else {
                        new_board[PieceNum::WhitePawn as usize] += 1 << (i + 8);
                    }
                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 8)), new_board));

                    if 7 <= i && i < 16 && occupied_board & 1 << (i + 16) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                        new_board[PieceNum::WhitePawn as usize] += 1 << (i + 16);
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 + 16)), new_board));
                    }
                }

                let mut possible_attacks = constants.pawn_reach[0][i as usize] & opposite_pieces;
                while possible_attacks != 0 {
                    let j = possible_attacks.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhitePawn as usize] -= 1 << i;
                    new_board[PieceNum::WhitePawn as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    possible_attacks -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Rooks
            let mut tmp = board[PieceNum::WhiteRook as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteRook as usize] -= 1 << i;
                    new_board[PieceNum::WhiteRook as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Knights
            let mut tmp = board[PieceNum::WhiteKnight as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.knight_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteKnight as usize] -= 1 << i;
                    new_board[PieceNum::WhiteKnight as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Bishops
            let mut tmp = board[PieceNum::WhiteBishop as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteBishop as usize] -= 1 << i;
                    new_board[PieceNum::WhiteBishop as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Queen
            let mut tmp = board[PieceNum::WhiteQueen as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let reachable = reachable_straight | reachable_diagonals;
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteQueen as usize] -= 1 << i;
                    new_board[PieceNum::WhiteQueen as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // King
            let mut tmp = board[PieceNum::WhiteKing as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.king_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::WhiteKing as usize] -= 1 << i;
                    new_board[PieceNum::WhiteKing as usize] += 1 << j;

                    new_board[PieceNum::BlackPawn as usize] &=  !(1 << j);
                    new_board[PieceNum::BlackRook as usize] &= !(1 << j);
                    new_board[PieceNum::BlackKnight as usize] &= !(1 << j);
                    new_board[PieceNum::BlackBishop as usize] &= !(1 << j);
                    new_board[PieceNum::BlackQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }
        },
        PieceColor::Black => {
            // Pawns
            let mut tmp = board[PieceNum::BlackPawn as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                if occupied_board & 1 << (i - 8) == 0 {
                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                    if 0 <= i - 8 && i - 8 < 8 {
                        new_board[PieceNum::BlackQueen as usize] += 1 << (i - 8);
                    } else {
                        new_board[PieceNum::BlackPawn as usize] += 1 << (i - 8);
                    }
                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 8)), new_board));

                    if 48 <= i && i < 56 && occupied_board & 1 << (i - 16) == 0 {
                        let mut new_board = board.clone();
                        new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                        new_board[PieceNum::BlackPawn as usize] += 1 << (i - 16);
                        possible_moves.push(((num_to_pos(i as u64), num_to_pos(i as u64 - 16)), new_board));
                    }
                }

                let mut possible_attacks = constants.pawn_reach[1][i as usize] & opposite_pieces;
                while possible_attacks != 0 {
                    let j = possible_attacks.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackPawn as usize] -= 1 << i;
                    new_board[PieceNum::BlackPawn as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    possible_attacks -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Rooks
            let mut tmp = board[PieceNum::BlackRook as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackRook as usize] -= 1 << i;
                    new_board[PieceNum::BlackRook as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Knights
            let mut tmp = board[PieceNum::BlackKnight as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.knight_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackKnight as usize] -= 1 << i;
                    new_board[PieceNum::BlackKnight as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Bishops
            let mut tmp = board[PieceNum::BlackBishop as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackBishop as usize] -= 1 << i;
                    new_board[PieceNum::BlackBishop as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // Queen
            let mut tmp = board[PieceNum::BlackQueen as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable_straight = constants.rook_reach[i as usize].get(&(constants.row_and_column_mask[i as usize] & occupied_board)).unwrap();
                let reachable_diagonals = constants.bishop_reach[i as usize].get(&(constants.diagonal_mask[i as usize] & occupied_board)).unwrap();
                let reachable = reachable_straight | reachable_diagonals;
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackQueen as usize] -= 1 << i;
                    new_board[PieceNum::BlackQueen as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }

            // King
            let mut tmp = board[PieceNum::BlackKing as usize];
            while tmp != 0 {
                let i = tmp.trailing_zeros();

                let reachable = constants.king_reach[i as usize];
                let mut moveable = reachable & !own_pieces;

                while moveable != 0 {
                    let j = moveable.trailing_zeros();

                    let mut new_board = board.clone();
                    new_board[PieceNum::BlackKing as usize] -= 1 << i;
                    new_board[PieceNum::BlackKing as usize] += 1 << j;

                    new_board[PieceNum::WhitePawn as usize] &=  !(1 << j);
                    new_board[PieceNum::WhiteRook as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteKnight as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteBishop as usize] &= !(1 << j);
                    new_board[PieceNum::WhiteQueen as usize] &= !(1 << j);

                    possible_moves.push(((num_to_pos(i as u64), num_to_pos(j as u64)), new_board));
                    moveable -= 1 << j;
                }

                tmp -= 1 << i;
            }
        }
    }

    // Validate
    let mut res = Vec::new();
    for mov in possible_moves {
        let b = mov.1;

        if !is_in_check(&b, color, &constants) {
            res.push(mov);
        }
    }

    res
}

fn get_piece_str(board: &BitBoard, letter: u64, number: u64) -> &str{
    let num = 1 << pos_to_num(letter, number);

    if board[PieceNum::WhitePawn as usize] & num == num {
        "\u{265F}"
    } else if board[PieceNum::WhiteRook as usize] & num == num {
        "\u{265C} "
    } else if board[PieceNum::WhiteKnight as usize] & num == num {
        "\u{265E} "
    } else if board[PieceNum::WhiteBishop as usize] & num == num {
        "\u{265D} "
    } else if board[PieceNum::WhiteQueen as usize] & num == num {
        "\u{265B} "
    } else if board[PieceNum::WhiteKing as usize] & num == num {
        "\u{265A} "
    } else if board[PieceNum::BlackPawn as usize] & num == num {
        "\u{2659} "
    } else if board[PieceNum::BlackRook as usize] & num == num {
        "\u{2656} "
    } else if board[PieceNum::BlackKnight as usize] & num == num {
        "\u{2658} "
    } else if board[PieceNum::BlackBishop as usize] & num == num {
        "\u{2657} "
    } else if board[PieceNum::BlackQueen as usize] & num == num {
        "\u{2655} "
    } else if board[PieceNum::BlackKing as usize] & num == num {
        "\u{2654} "
    } else {
        "  "
    }
}

pub fn get_bitboard_ascii(board: &BitBoard) -> String {
    let mut string = String::with_capacity(844);
    
    for i in (0..8).rev() {
        string.push_str("  +----+----+----+----+----+----+----+----+\n");
        if let Some(res) = std::char::from_digit(1 + i as u32, 10) {
            string.push(res);
            string.push(' ');
        }
        for j in 0..8 {
            string.push_str("| ");
            string.push_str(get_piece_str(&board, j as u64, i as u64));
            string.push(' ');
        }
        string.push_str("|\n");
    }
    string.push_str("  +----+----+----+----+----+----+----+----+\n");
    string.push_str("    A    B    C    D    E    F    G    H");

    string
}
