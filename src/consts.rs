pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;

pub const ROWS: u32 = 8;
pub const COLS: u32 = 8;

pub const SQUARE_WIDTH: u32 = WIDTH / ROWS;
pub const SQUARE_HEIGHT: u32 = HEIGHT / COLS;

pub const IMAGE_WIDTH: i32 = 2000;
pub const IMAGE_HEIGHT: i32 = 668;
pub const TEXTURE_DATA: &[u8; 89806] = include_bytes!("../assets/Pieces.png");

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
