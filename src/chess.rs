#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}
