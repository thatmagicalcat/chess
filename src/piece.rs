use sfml::graphics::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn as_color(&self) -> Color {
        match self {
            PieceColor::White => Color::rgb(170, 150, 150),
            PieceColor::Black => Color::rgb(90, 70, 70),
        }
    }

    #[inline(always)]
    pub fn is_opposite(&self, other: &Self) -> bool {
        self != other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,

    None,
}

impl Piece {
    pub fn from_u8(n: u8) -> Option<Self> {
        if n > 5 {
            return None;
        }

        Some(match n {
            0 => Self::King,
            1 => Self::Queen,
            2 => Self::Bishop,
            3 => Self::Knight,
            4 => Self::Rook,
            5 => Self::Pawn,

            _ => unreachable!(),
        })
    }
}
