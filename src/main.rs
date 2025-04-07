use board::Board;
use glfw::*;
use glow::HasContext;

mod bitboard;
mod board;
mod grid_renderer;
mod piece;
mod piece_renderer;
mod program_manager;

use program_manager::Program;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // OpenGL 4.2 core
    glfw.window_hint(WindowHint::ContextVersion(4, 2));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    glfw.window_hint(WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, "chess", WindowMode::Windowed)
        .unwrap();

    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);

    let mut gl = unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s)) };

    println!("OpenGL {}", unsafe {
        gl.get_parameter_string(glow::VERSION)
    });

    unsafe {
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        gl.viewport(0, 0, WIDTH as _, HEIGHT as _);

        #[cfg(not(target_os = "macos"))]
        gl.debug_message_callback(|_source, _gltype, id, severity, msg| {
            println!(
                "GL CALLBACK: {} severity = {}, message = {}",
                id, severity, msg
            );
        });
    }

    // Adaptive V-Sync
    glfw.set_swap_interval(SwapInterval::Adaptive);

    let mut board = Board::new(&gl);

    while !window.should_close() {
        glfw.poll_events();
        for (_, window_event) in glfw::flush_messages(&events) {
            #[allow(clippy::single_match)]
            match window_event {
                WindowEvent::Close => window.set_should_close(true),
                WindowEvent::MouseButton(MouseButtonLeft, Action::Press, ..) => {
                    const STEP: f64 = 1.0 / 8.0;

                    let (w, h) = window.get_size();
                    let (x, y) = window.get_cursor_pos();
                    let (x, y) = (x / w as f64, y / h as f64);

                    let (col, row) = (x / STEP, y / STEP);
                    let (col, row) = (col as i32, 7 - row as i32);
                    dbg!(col, row);

                    board.set_active_square(Some((col, row)));
                }

                _ => {}
            }
        }

        unsafe {
            gl.clear_color(0.0, 0.2, 0.2, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            board.render();
        }

        window.swap_buffers();
    }
}
