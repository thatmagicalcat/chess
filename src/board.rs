use crate::grid_renderer::GridRenderer;
use crate::piece::*;
use crate::piece_renderer::PieceRenderer;

pub const ROWS: u32 = 8;
pub const COLS: u32 = 8;
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub struct Board<'a> {
    gl: &'a glow::Context,
    grid: GridRenderer<'a>,
    squares: grid::Grid<Option<Piece>>,
    piece_renderer: PieceRenderer<'a>,
}

impl<'a> Board<'a> {
    pub fn new(gl: &'a glow::Context) -> Self {
        let grid = GridRenderer::new(gl);
        let squares = Self::parse_fen(START_FEN);
        let piece_renderer = PieceRenderer::new(gl);

        dbg!(squares.get(0, 0).unwrap());

        Self {
            gl,
            grid,
            squares,
            piece_renderer,
        }
    }

    pub fn render(&self) {
        self.grid.render(Some((0, 0)));
        self.piece_renderer.render(&self.squares);
    }

    pub fn parse_fen(fen: &str) -> grid::Grid<Option<Piece>> {
        let mut squares = grid::Grid::init(ROWS as _, COLS as _, None);

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
                    let piece_ty = match p.to_ascii_lowercase() {
                        'k' => PieceTy::King,
                        'q' => PieceTy::Queen,
                        'b' => PieceTy::Bishop,
                        'n' => PieceTy::Knight,
                        'r' => PieceTy::Rook,
                        'p' => PieceTy::Pawn,

                        x => unreachable!("unknown char: {x}"),
                    };

                    *squares.get_mut(row, col).unwrap() = Some(Piece {
                        ty: piece_ty,
                        color: if is_black {
                            PieceColor::Black
                        } else {
                            PieceColor::White
                        },
                    });

                    col += 1;
                }
            }
        }

        squares
    }
}
