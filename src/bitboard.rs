use crate::piece::*;

const FILE_A: u64 = 0x0101010101010101;
const FILE_B: u64 = 0x0202020202020202;
const FILE_C: u64 = 0x0404040404040404;
const FILE_D: u64 = 0x0808080808080808;
const FILE_E: u64 = 0x1010101010101010;
const FILE_F: u64 = 0x2020202020202020;
const FILE_G: u64 = 0x4040404040404040;
const FILE_H: u64 = 0x8080808080808080;

const RANK_1: u64 = 0x00000000000000FF;
const RANK_2: u64 = 0x000000000000FF00;
const RANK_7: u64 = 0x00FF000000000000;
const RANK_8: u64 = 0xFF00000000000000;

pub const MAX_MOVES: usize = 256;
pub struct MoveList {
    /// from << 8 | to
    array: [u16; MAX_MOVES],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            array: [0; MAX_MOVES],
            len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// in square index
    pub fn add(&mut self, from: u8, to: u8) {
        self.array[self.len] = (from as u16) << 8 | to as u16;
        self.len += 1;
    }

    /// Returns the possible moves of a piece located at a certain square
    pub fn get_moves(&self, from: u8) -> u64 {
        self.array
            .iter()
            .filter(|&&i| (i >> 8) as u8 == from)
            .fold(0, |acc, &i| acc | (1_u64 << (i & u8::MAX as u16)))
    }

    pub fn as_slice(&self) -> &[u16] {
        &self.array[..self.len]
    }
}

#[derive(Debug, Default)]
pub struct BitBoard {
    bits: [[u64; 6]; 2],
}

impl BitBoard {
    pub fn get_piece_at(&self, square: u8) -> Option<Piece> {
        let mask = 1u64 << square;
        self.bits
            .iter()
            .flatten()
            .enumerate()
            .find(|(_index, bits)| **bits & mask != 0)
            .map(|(index, _bits)| Piece::from_square_index(index))?
    }

    pub fn get_color_pieces(&self, color: PieceColor) -> u64 {
        self.bits[color as u8 as usize]
            .iter()
            .fold(0, |acc, i| acc | *i)
    }

    pub const fn get_piece_bits(&self, piece: Piece) -> u64 {
        let (a, b) = piece.get_bitboard_index();
        self.bits[a][b]
    }

    pub const fn get_piece_bits_mut(&mut self, piece: Piece) -> &mut u64 {
        let (a, b) = piece.get_bitboard_index();
        &mut self.bits[a][b]
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut bb = Self::default();

        let mut row = 0;
        let mut col = 0;

        for ch in fen.chars() {
            match ch {
                n if n.is_ascii_digit() => col += n.to_digit(10).unwrap() as usize,

                '/' => {
                    col = 0;
                    row += 1;
                }

                p => {
                    let is_black = p.is_lowercase();
                    let color = if is_black {
                        PieceColor::Black
                    } else {
                        PieceColor::White
                    };

                    let ty = match p.to_ascii_lowercase() {
                        'k' => PieceTy::King,
                        'q' => PieceTy::Queen,
                        'b' => PieceTy::Bishop,
                        'n' => PieceTy::Knight,
                        'r' => PieceTy::Rook,
                        'p' => PieceTy::Pawn,

                        x => unreachable!("unknown char: {x}"),
                    };

                    let shift = row * 8 + col;
                    *bb.get_piece_bits_mut(Piece { ty, color }) |= 1 << shift;

                    col += 1;
                }
            }
        }

        bb
    }

    pub fn generate_moves(&self, color: PieceColor) -> MoveList {
        let mut move_list = MoveList::new();

        self.generate_king_moves(color, &mut move_list);
        self.generate_pawn_moves(color, &mut move_list);
        self.generate_knight_moves(color, &mut move_list);

        move_list
    }

    pub fn generate_pawn_moves(&self, color: PieceColor, move_list: &mut MoveList) {
        let friendly_bits = self.get_color_pieces(color);
        let opposite_bits = self.get_color_pieces(color.opposite());

        let pawn_bits = self.get_piece_bits(Piece {
            ty: PieceTy::Pawn,
            color,
        });

        // square index [0, 63]
        let mut pawns = pawn_bits;

        // for all pawns
        while pawns != 0 {
            let from = pawns.trailing_zeros();
            let mut move_targets =
                self.generate_pawn_attacks(from, opposite_bits, color) & !friendly_bits;

            while move_targets != 0 {
                let to = move_targets.trailing_zeros();
                move_list.add(from as _, to as _);

                // clear LSB
                move_targets &= move_targets - 1;
            }

            // clear LSB
            pawns &= pawns - 1;
        }
    }

    fn generate_pawn_attacks(&self, square: u32, opposite_bits: u64, color: PieceColor) -> u64 {
        let pawn = 1_u64 << square;

        match color {
            PieceColor::White => {
                let up = pawn << 8 & !opposite_bits;
                let double_push = up << 16 & RANK_2 & !opposite_bits;

                let top_left = ((pawn << 7) & !FILE_H) & opposite_bits;
                let top_right = ((pawn << 9) & !FILE_A) & opposite_bits;

                up | top_left | top_right | double_push
            }

            PieceColor::Black => {
                let down = pawn >> 8 & !opposite_bits;
                let double_push = down >> 16 & RANK_7 & !opposite_bits;

                let bottom_right = ((pawn >> 7) & !FILE_A) & opposite_bits;
                let bottom_left = ((pawn >> 9) & !FILE_H) & opposite_bits;

                down | bottom_left | bottom_right | double_push
            }
        }
    }

    pub fn generate_king_moves(&self, color: PieceColor, move_list: &mut MoveList) {
        let friendly_bits = self.get_color_pieces(color);
        let king_bits = self.get_piece_bits(Piece {
            ty: PieceTy::King,
            color,
        });

        assert!(king_bits != 0, "not a valid chess position");

        // square index [0, 63]
        let from = king_bits.trailing_zeros();
        let mut move_targets = self.generate_king_attacks(from) & !friendly_bits;

        while move_targets != 0 {
            let to = move_targets.trailing_zeros();
            move_list.add(from as _, to as _);

            // clear LSB
            move_targets &= move_targets - 1;
        }
    }

    const fn generate_king_attacks(&self, square: u32) -> u64 {
        // pre-calculate all possible attacks at compile time
        const KING_MOVES: [u64; 64] = {
            let mut king_moves = [0_u64; 64];
            let mut square = 0;

            while square != 64 {
                let pos = 1_u64 << square;

                king_moves[square] = ((pos << 1) & !FILE_A)  // right
                    | (pos >> 1) & !FILE_H  // left
                    | pos << 8              // up
                    | pos >> 8              // down
                    | (pos << 9) & !FILE_A  // top right
                    | (pos << 7) & !FILE_H  // top left
                    | (pos >> 7) & !FILE_A  // bottom right
                    | (pos >> 9) & !FILE_H; // bottom left

                square += 1;
            }

            king_moves
        };

        assert!(square < 64);
        KING_MOVES[square as usize]
    }

    pub fn generate_knight_moves(&self, color: PieceColor, move_list: &mut MoveList) {
        let friendly_bits = self.get_color_pieces(color);
        let knight_bits = self.get_piece_bits(Piece {
            ty: PieceTy::Knight,
            color,
        });

        // square index [0, 63]
        let mut knights = knight_bits;

        // for all knights
        while knights != 0 {
            let from = knights.trailing_zeros();
            let mut move_targets = self.generate_knight_attacks(from) & !friendly_bits;

            while move_targets != 0 {
                let to = move_targets.trailing_zeros();
                move_list.add(from as _, to as _);

                // clear LSB
                move_targets &= move_targets - 1;
            }

            // clear LSB
            knights &= knights - 1;
        }
    }

    fn generate_knight_attacks(&self, square: u32) -> u64 {
        const KNIGHT_MOVES: [u64; 64] = {
            let mut knight_moves = [0_u64; 64];
            let mut square = 0;

            while square != 64 {
                let pos = 1_u64 << square;

                knight_moves[square] |= (pos & !(FILE_A | FILE_B)) << 6
                    | (pos & !FILE_A) << 15
                    | (pos & !(FILE_H | FILE_G)) << 10
                    | (pos & !FILE_H) << 17
                    | (pos & !(FILE_H | FILE_G)) >> 6
                    | (pos & !FILE_H) >> 15
                    | (pos & !(FILE_A | FILE_B)) >> 10
                    | (pos & !FILE_A) >> 17;

                square += 1;
            }

            knight_moves
        };

        assert!(square < 64);
        KNIGHT_MOVES[square as usize]
    }

    // static bitboard_t KNIGHT_MOVES[64];

    // void _chess_knight_moves_init(void) {
    // for (uint8_t i = 0; i < 64; i++) {
    //     bitboard_t pos = 1ULL << i;

    //     KNIGHT_MOVES[i] |= ((pos & ~(FILE_A | FILE_B)) << 6);
    //     KNIGHT_MOVES[i] |= ((pos & ~FILE_A) << 15);
    //     KNIGHT_MOVES[i] |= ((pos & ~(FILE_H | FILE_G)) << 10);
    //     KNIGHT_MOVES[i] |= ((pos & ~FILE_H) << 17);

    //     KNIGHT_MOVES[i] |= ((pos & ~(FILE_H | FILE_G)) >> 6);
    //     KNIGHT_MOVES[i] |= ((pos & ~FILE_H) >> 15);
    //     KNIGHT_MOVES[i] |= ((pos & ~(FILE_A | FILE_B)) >> 10);
    //     KNIGHT_MOVES[i] |= ((pos & ~FILE_A) >> 17);
    // }
    // }
}
