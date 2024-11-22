use crate::chess_piece::ChessPiece;
use crate::color::Color;

/// Represents a pawn chess piece
pub struct Pawn {
    color: Color,
}

impl ChessPiece for Pawn {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_as_char(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
