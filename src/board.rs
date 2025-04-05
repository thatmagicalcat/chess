use crate::bitboard::{BitBoard, MoveList};
use crate::grid_renderer::GridRenderer;
use crate::piece::Piece;
use crate::piece_renderer::PieceRenderer;

pub const ROWS: u32 = 8;
pub const COLS: u32 = 8;
pub const START_FEN: &str = "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr";

pub struct Board<'a> {
    gl: &'a glow::Context,
    grid: GridRenderer<'a>,
    bitboard: BitBoard,
    piece_renderer: PieceRenderer<'a>,

    move_list: MoveList,
    active_piece: Option<(i32, i32)>,
    active_piece_moves: u64,
}

impl<'a> Board<'a> {
    pub fn new(gl: &'a glow::Context) -> Self {
        let grid = GridRenderer::new(gl);
        let piece_renderer = PieceRenderer::new(gl);
        let bitboard = BitBoard::from_fen(START_FEN);

        Self {
            gl,
            grid,
            bitboard,
            piece_renderer,

            active_piece: None,
            move_list: MoveList::new(),
            active_piece_moves: 0,
        }
    }

    fn update_active_piece_moves(&mut self) {
        if let Some((col, row)) = self.active_piece {
            let active_square = (row * 8 + col) as u8;
            if let Some(Piece { color, .. }) = self.bitboard.get_piece_at(active_square) {
                self.move_list = self.bitboard.generate_moves(color);
                self.active_piece_moves = self.move_list.get_moves(active_square);

                print!("Piece: {:?}, ", self.bitboard.get_piece_at((row * 8 + col) as u8));
                println!("{:064b}", self.active_piece_moves);
            }
        }
    }

    pub fn set_active_square(&mut self, active_piece: Option<(i32, i32)>) {
        self.active_piece = active_piece;
        self.update_active_piece_moves();
    }

    pub fn render(&self) {
        // NOTE: (column, row)
        self.grid.render(self.active_piece, self.active_piece_moves);

        self.piece_renderer.render(&self.bitboard);
    }
}
