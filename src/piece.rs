#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub ty: PieceTy,
    pub color: PieceColor,
}

impl Piece {
    /// Returns the texture index in OpenGL TEXTURE_2D_ARRAY
    pub fn get_texture_index(&self) -> u8 {
        self.ty as u8 + self.color as u8 * 6
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PieceTy {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl PieceTy {
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
