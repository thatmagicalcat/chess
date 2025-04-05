#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub ty: PieceTy,
    pub color: PieceColor,
}

impl Piece {
    pub const ALL_PIECES: [Piece; 12] = [
        Piece::white(PieceTy::King),
        Piece::white(PieceTy::Queen),
        Piece::white(PieceTy::Bishop),
        Piece::white(PieceTy::Knight),
        Piece::white(PieceTy::Rook),
        Piece::white(PieceTy::Pawn),
        Piece::black(PieceTy::King),
        Piece::black(PieceTy::Queen),
        Piece::black(PieceTy::Bishop),
        Piece::black(PieceTy::Knight),
        Piece::black(PieceTy::Rook),
        Piece::black(PieceTy::Pawn),
    ];

    /// Returns the texture index in OpenGL TEXTURE_2D_ARRAY
    pub const fn get_texture_index(&self) -> u8 {
        self.ty as u8 + self.color as u8 * 6
    }

    pub const fn get_bitboard_index(&self) -> (usize, usize) {
        (self.color as u8 as usize, self.ty as u8 as usize)
    }

    pub fn from_square_index(idx: usize) -> Option<Self> {
        const PIECE_COUNT: usize = 6;

        if idx < PIECE_COUNT {
            // white piece
            PieceTy::from_u8(idx as _).map(Piece::white)
        } else {
            // black piece
            PieceTy::from_u8((idx - PIECE_COUNT) as _).map(Piece::black)
        }
    }

    #[inline]
    pub const fn black(ty: PieceTy) -> Self {
        Piece {
            ty,
            color: PieceColor::Black,
        }
    }

    #[inline]
    pub const fn white(ty: PieceTy) -> Self {
        Piece {
            ty,
            color: PieceColor::White,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
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
    pub const fn from_u8(n: u8) -> Option<Self> {
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
