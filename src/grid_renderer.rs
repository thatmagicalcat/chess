use glow::HasContext;

use crate::{Program, bitboard::BitBoard, piece::Piece};

#[rustfmt::skip]
const BOARD_VERTICES: [f32; 12] = [
    // Triangle 1
    -1.0,  1.0, // top left
     1.0,  1.0, // top right
     1.0, -1.0, // bottom right

     // Triangle 2
     1.0, -1.0, // bottom right
    -1.0, -1.0, // bottom left
    -1.0,  1.0, // top left
];

pub struct GridRenderer<'a> {
    gl: &'a glow::Context,
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    program: Program<'a>,
}

impl<'a> GridRenderer<'a> {
    pub fn new(gl: &'a glow::Context) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };
        let vbo = unsafe { gl.create_buffer().unwrap() };

        unsafe {
            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&BOARD_VERTICES),
                glow::STATIC_DRAW,
            );

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                std::mem::size_of::<f32>() as i32 * 2,
                0,
            );

            gl.bind_vertex_array(None);
        }

        let program =
            Program::from_str(gl, include_str!("shaders/grid.glsl"), "vertex", "fragment").unwrap();

        Self {
            gl,
            vao,
            vbo,
            program,
        }
    }

    /// col, row
    pub fn render(&self, active_piece: Option<(i32, i32)>, piece_moves: u64) {
        let (col, row) = active_piece.unwrap_or((-1, -1));

        unsafe {
            /*
             *
             * To my future self and other people reading it: I apologise for all the shit
             * code following this comment and the code which is inside the grid shader,
             * maybe I don't know enough graphics programming at the time or writing this.
             * I sincerely apologise for all the headache it is going to cause in the future.
             *
             */

            let ms_part = ((piece_moves & ((u32::MAX as u64) << 32)) >> 32) as u32;
            let ls_part = (piece_moves & u32::MAX as u64) as u32;

            self.program.use_program();

            self.gl.uniform_2_i32(
                self.program.get_uniform_location("active_piece").as_ref(),
                col,
                row,
            );

            self.gl.uniform_2_u32(
                self.program.get_uniform_location("piece_moves").as_ref(),
                ms_part,
                ls_part,
            );

            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}

impl Drop for GridRenderer<'_> {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_vertex_array(self.vao);
        }
    }
}
