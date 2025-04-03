use sfml::{graphics::*, window::*};

mod board;
mod consts;
mod piece;

use consts::*;

fn main() {
    // print hello world
    let window_setttings = ContextSettings {
        antialiasing_level: 8,
        ..Default::default()
    };

    let mut window =
        RenderWindow::new((WIDTH, HEIGHT), "Chess", Style::CLOSE, &window_setttings).unwrap();

    window.set_vertical_sync_enabled(true);

    let mut t = Texture::new().unwrap();
    t.load_from_memory(TEXTURE_DATA, Rect::new(0, 0, IMAGE_WIDTH, IMAGE_HEIGHT))
        .unwrap();

    let mut board = board::Board::new(&t);
    board.parse_fen(START_FEN);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            board.handle_event(event);

            #[allow(clippy::single_match)]
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        window.clear(Color::WHITE);
        board.draw(&mut window);
        window.display();
    }
}
