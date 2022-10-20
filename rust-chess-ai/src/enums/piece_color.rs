
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White = 0,
    Black = 1
}

impl PieceColor {
    pub fn opposite_color(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }
}
