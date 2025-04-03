use std::collections::HashMap;

use grid::Grid;
use sfml::graphics::*;
use sfml::window::Event;

use crate::consts::*;
use crate::piece::*;

pub struct Board<'a> {
    squares: Grid<(Piece, PieceColor)>,
    square_drawing_shape: RectangleShape<'a>,
    texture_drawing_shape: RectangleShape<'a>,
    move_drawing_shape: CircleShape<'a>,
    texture_rect: HashMap<(Piece, PieceColor), Rect<i32>>,
    active_cell: (i32, i32),
    moves: Vec<(i32, i32)>,
    active_piece_color: Option<PieceColor>,
}

impl<'a> Board<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        Self {
            active_piece_color: None,
            moves: Vec::new(),
            active_cell: (-1, -1),
            squares: Grid::init(ROWS as _, COLS as _, (Piece::None, PieceColor::White)),
            texture_rect: {
                let mut map = HashMap::with_capacity(12);

                const SUB_TXR_HEIGHT: i32 = IMAGE_HEIGHT / 2; // 2 cols
                const SUB_TXR_WIDTH: i32 = IMAGE_WIDTH / 6; // 6 rows

                for row in 0..2 {
                    let color = [PieceColor::White, PieceColor::Black][row];
                    for col in 0..6 {
                        let piece = Piece::from_u8(col as _).unwrap();

                        map.insert(
                            (piece, color),
                            Rect::new(
                                col * SUB_TXR_WIDTH,
                                row as i32 * SUB_TXR_HEIGHT,
                                SUB_TXR_WIDTH,
                                SUB_TXR_HEIGHT,
                            ),
                        );
                    }
                }

                map
            },

            square_drawing_shape: {
                let mut s = RectangleShape::new();
                s.set_size((SQUARE_WIDTH as _, SQUARE_HEIGHT as _));
                s.set_outline_thickness(0.);
                s
            },

            texture_drawing_shape: {
                let mut s = RectangleShape::new();
                s.set_size((SQUARE_WIDTH as _, SQUARE_HEIGHT as _));
                s.set_outline_thickness(0.);
                s.set_texture(texture, true);
                s
            },

            move_drawing_shape: {
                let mut c = CircleShape::new(8., 20);
                c.set_origin((
                    (SQUARE_WIDTH as f32 / 2. - 8.) * -1.,
                    (SQUARE_HEIGHT as f32 / 2. - 8.) * -1.,
                ));
                c.set_fill_color(Color::GREEN);
                c
            },
        }
    }

    pub fn parse_fen(&mut self, fen: &str) {
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
                    let piece = match p.to_ascii_lowercase() {
                        'k' => Piece::King,
                        'q' => Piece::Queen,
                        'b' => Piece::Bishop,
                        'n' => Piece::Knight,
                        'r' => Piece::Rook,
                        'p' => Piece::Pawn,

                        x => unreachable!("unknown char: {x}"),
                    };

                    self.squares[row][col] = (
                        piece,
                        if is_black {
                            PieceColor::Black
                        } else {
                            PieceColor::White
                        },
                    );

                    col += 1;
                }
            }
        }
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        for row in 0..self.squares.rows() {
            for (col_idx, col) in self.squares.iter_col(row).enumerate() {
                let is_white_square = (row + col_idx) % 2 == 0;

                self.square_drawing_shape.set_position((
                    (row as u32 * SQUARE_WIDTH) as _,
                    (col_idx as u32 * SQUARE_HEIGHT) as _,
                ));

                self.texture_drawing_shape.set_position((
                    (row as u32 * SQUARE_WIDTH) as _,
                    (col_idx as u32 * SQUARE_HEIGHT) as _,
                ));

                self.square_drawing_shape.set_fill_color(if is_white_square {
                    PieceColor::White.as_color()
                } else {
                    PieceColor::Black.as_color()
                });

                let (x, y) = self.active_cell;

                if row as i32 == y && col_idx as i32 == x {
                    let color = Color::rgb(50, 200, 50);
                    self.square_drawing_shape.set_fill_color(color);
                }

                window.draw(&self.square_drawing_shape);

                // Draw possible moves (if a piece is selected)
                for (row, col) in self.moves.iter() {
                    self.move_drawing_shape.set_fill_color(Color::GREEN);

                    if self.squares.get(*row as usize, *col as usize).is_none() {
                        continue;
                    }

                    if let Some((p, piece_color)) = self.squares.get(*row as usize, *col as usize) {
                        if *p != Piece::None
                            && self
                                .active_piece_color
                                .is_some_and(|i| i.is_opposite(piece_color))
                        {
                            self.move_drawing_shape.set_fill_color(Color::RED);
                        }
                    }

                    self.move_drawing_shape.set_position((
                        (*col as u32 * SQUARE_WIDTH) as _,
                        (*row as u32 * SQUARE_HEIGHT) as _,
                    ));

                    window.draw(&self.move_drawing_shape);
                }

                if !matches!(col.0, Piece::None) {
                    self.texture_drawing_shape
                        .set_texture_rect(self.texture_rect[&(col.0, col.1)]);

                    window.draw(&self.texture_drawing_shape);
                }

                // self.move_draw_rect.set_position((
                //     (row as u32 * SQUARE_WIDTH) as _,
                //     (col_idx as u32 * SQUARE_HEIGHT) as _,
                // ));
                // window.draw(&self.move_draw_rect);
            }
        }
    }

    fn get_active_piece(&mut self) -> Option<&mut (Piece, PieceColor)> {
        self.squares
            .get_mut(self.active_cell.0 as usize, self.active_cell.1 as usize)
    }

    pub fn handle_event(&mut self, event: sfml::window::Event) {
        if let Event::MouseButtonPressed { x, y, .. } = event {
            let Square((x, y), (clicked_piece, clicked_piece_color)) = self.get_square(x, y);

            if self.active_piece_color.is_none() && clicked_piece != Piece::None {
                self.active_cell = (x, y);

                let moves = self.calc_moves(x as _, y as _);

                self.moves.clear();
                self.moves = moves;
                self.active_piece_color = Some(clicked_piece_color);
            } else if self.active_piece_color.is_some() && clicked_piece != Piece::None {
                if self.moves.contains(&(x, y)) && clicked_piece_color.is_opposite(self.active_piece_color.as_ref().unwrap()) {
                    self.squares[x as usize][y as usize] = *self.get_active_piece().unwrap();
                    self.get_active_piece().unwrap().0 = Piece::None;
                }

                self.active_cell = (-1, -1);
                self.active_piece_color = None;
                self.moves.clear();
            } else {
                if self.moves.contains(&(x, y)) {
                    let (a_x, a_y) = self.active_cell;

                    self.squares[x as usize][y as usize] = self.squares[a_x as usize][a_y as usize];
                    self.squares[a_x as usize][a_y as usize].0 = Piece::None;
                }

                self.active_cell = (-1, -1);
                self.active_piece_color = None;
                self.moves.clear();
            }
        }
    }

    fn get_square(&self, x: i32, y: i32) -> Square {
        // column of the cell
        let col_idx = x / SQUARE_WIDTH as i32;

        // row of the cell
        let row_idx = y / SQUARE_HEIGHT as i32;

        Square(
            (row_idx, col_idx),
            self.squares[row_idx as usize][col_idx as usize],
        )
    }

    fn calc_moves(&self, row: i32, col: i32) -> Vec<(i32, i32)> {
        let mut moves = vec![];
        let (current_piece, current_piece_color) = dbg!(self.squares[row as usize][col as usize]);

        match current_piece {
            Piece::King => {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let new_row = row + i;
                        let new_col = col + j;

                        if let Some((p, c)) = self.squares.get(new_row as _, new_col as _) {
                            if *p == Piece::None || c.is_opposite(&current_piece_color) {
                                moves.push((new_row, new_col));
                            }
                        }
                    }
                }
            }

            Piece::Queen => {
                // check all the pieces in the left row
                for i in (0..row).rev() {
                    let (p, c) = self.squares.get(i as _, col as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, col));
                        }

                        break;
                    }

                    moves.push((i, col));
                }

                // check all the pieces in the right row
                for i in row + 1..ROWS as i32 {
                    let (p, c) = self.squares.get(i as _, col as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, col));
                        }

                        break;
                    }

                    moves.push((i, col));
                }

                // check all the pieces in the top column
                for i in (0..col).rev() {
                    let (p, c) = self.squares.get(row as _, i as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((row, i));
                        }

                        break;
                    }

                    moves.push((row, i));
                }

                // check all the pieces in the bottom column
                for i in col + 1..COLS as i32 {
                    let (p, c) = self.squares.get(row as _, i as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((row, i));
                        }

                        break;
                    }

                    moves.push((row, i));
                }

                // Check all the pieces in the top left diagonal
                for i in (0..row).rev() {
                    let j = col - (row - i);

                    if j < 0 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }

                // Check all the pieces in the top right diagonal
                for i in (0..row).rev() {
                    let j = col + (row - i);

                    if j >= COLS as i32 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }

                // check all the pieces in the bottom left diagonal
                for i in row + 1..ROWS as i32 {
                    let j = col - (i - row);

                    if j < 0 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }

                // check all the pieces in the bottom right diagonal
                for i in row + 1..ROWS as i32 {
                    let j = col + (i - row);

                    if j >= COLS as i32 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }
            }

            Piece::Bishop => {
                // Check all the pieces in the top left diagonal
                for i in (0..row).rev() {
                    let j = col - (row - i);

                    if j < 0 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }

                // Check all the pieces in the top right diagonal
                for i in (0..row).rev() {
                    let j = col + (row - i);

                    if j >= COLS as i32 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }

                // check all the pieces in the bottom left diagonal
                for i in row + 1..ROWS as i32 {
                    let j = col - (i - row);

                    if j < 0 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }

                // check all the pieces in the bottom right diagonal
                for i in row + 1..ROWS as i32 {
                    let j = col + (i - row);

                    if j >= COLS as i32 {
                        break;
                    }

                    let (p, c) = self.squares.get(i as _, j as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, j));
                        }

                        break;
                    }

                    moves.push((i, j));
                }
            }

            Piece::Knight => {
                let possible_moves = [
                    (row - 2, col - 1),
                    (row - 2, col + 1),
                    (row + 2, col - 1),
                    (row + 2, col + 1),
                    (row - 1, col - 2),
                    (row - 1, col + 2),
                    (row + 1, col - 2),
                    (row + 1, col + 2),
                ];

                for (r, c) in possible_moves.iter() {
                    if let Some((p, piece_color)) = self.squares.get(*r as _, *c as _) {
                        if *p == Piece::None || piece_color.is_opposite(&current_piece_color) {
                            moves.push((*r, *c));
                        }
                    }
                }
            }

            Piece::Rook => {
                // check all the pieces in the left row
                for i in (0..row).rev() {
                    let (p, c) = self.squares.get(i as _, col as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, col));
                        }

                        break;
                    }

                    moves.push((i, col));
                }

                // check all the pieces in the right row
                for i in row + 1..ROWS as i32 {
                    let (p, c) = self.squares.get(i as _, col as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((i, col));
                        }

                        break;
                    }

                    moves.push((i, col));
                }

                // check all the pieces in the top column
                for i in (0..col).rev() {
                    let (p, c) = self.squares.get(row as _, i as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((row, i));
                        }

                        break;
                    }

                    moves.push((row, i));
                }

                // check all the pieces in the bottom column
                for i in col + 1..COLS as i32 {
                    let (p, c) = self.squares.get(row as _, i as _).unwrap();

                    if *p != Piece::None {
                        if c.is_opposite(&current_piece_color) {
                            moves.push((row, i));
                        }

                        break;
                    }

                    moves.push((row, i));
                }
            }

            Piece::Pawn => {
                if current_piece_color == PieceColor::White {
                    if row == 6 {
                        if let Some((p, _)) = self.squares.get(row as usize - 1, col as usize) {
                            if *p == Piece::None {
                                moves.push((row - 1, col));
                            }
                        }

                        if let Some((p, _)) = self.squares.get(row as usize - 2, col as usize) {
                            if *p == Piece::None {
                                moves.push((row - 2, col));
                            }
                        }

                        if col > 0 {
                            if let Some((p, c)) =
                                self.squares.get(row as usize - 1, col as usize + 1)
                            {
                                if *p != Piece::None && c.is_opposite(&current_piece_color) {
                                    moves.push((row - 1, col + 1));
                                }
                            }
                        }
                    } else {
                        if row > 0 {
                            if let Some((p, _)) = self.squares.get(row as usize - 1, col as usize) {
                                if *p == Piece::None {
                                    moves.push((row - 1, col));
                                }
                            }
                        }

                        if col > 0 && row > 0 {
                            if let Some((p, c)) =
                                self.squares.get(row as usize - 1, col as usize - 1)
                            {
                                if *p != Piece::None && c.is_opposite(&current_piece_color) {
                                    moves.push((row - 1, col - 1));
                                }
                            }
                        }

                        if row > 0 {
                            if let Some((p, c)) =
                                self.squares.get(row as usize - 1, col as usize + 1)
                            {
                                if *p != Piece::None && c.is_opposite(&current_piece_color) {
                                    moves.push((row - 1, col + 1));
                                }
                            }
                        }
                    }
                } else if row == 1 {
                    if let Some((p, _)) = self.squares.get(row as usize + 1, col as usize) {
                        if *p == Piece::None {
                            moves.push((row + 1, col));
                        }
                    }

                    if let Some((p, _)) = self.squares.get(row as usize + 2, col as usize) {
                        if *p == Piece::None {
                            moves.push((row + 2, col));
                        }
                    }

                    if col > 0 {
                        if let Some((p, c)) =
                            self.squares.get(row as usize + 1, col as usize + 1)
                        {
                            if *p != Piece::None && c.is_opposite(&current_piece_color) {
                                moves.push((row + 1, col + 1));
                            }
                        }
                    }
                } else {
                    if row < 7 {
                        if let Some((p, _)) = self.squares.get(row as usize + 1, col as usize) {
                            if *p == Piece::None {
                                moves.push((row + 1, col));
                            }
                        }
                    }

                    if col > 0 && row < 7 {
                        if let Some((p, c)) =
                            self.squares.get(row as usize + 1, col as usize - 1)
                        {
                            if *p != Piece::None && c.is_opposite(&current_piece_color) {
                                moves.push((row + 1, col - 1));
                            }
                        }
                    }

                    if row < 7 {
                        if let Some((p, c)) =
                            self.squares.get(row as usize + 1, col as usize + 1)
                        {
                            if *p != Piece::None && c.is_opposite(&current_piece_color) {
                                moves.push((row + 1, col + 1));
                            }
                        }
                    }
                }
            }

            _ => {}
        }

        dbg!(&moves);

        moves
    }
}

#[derive(Debug, Clone, Copy)]
struct Square((i32, i32), (Piece, PieceColor));
