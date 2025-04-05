use std::io::Write;
use std::time::Instant;

use glow::HasContext;
use image::GenericImageView;

use crate::Program;
use crate::bitboard::BitBoard;
use crate::piece::Piece;

pub struct PieceRenderer<'a> {
    gl: &'a glow::Context,

    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,

    program: Program<'a>,
    texture_array_id: glow::NativeTexture,
}

impl<'a> PieceRenderer<'a> {
    pub fn new(gl: &'a glow::Context) -> Self {
        let (vao, vbo, ebo) = create_quad_mesh(gl);
        let texture_array_id = Self::load_texture(gl);
        let program =
            Program::from_str(gl, include_str!("shaders/piece.glsl"), "vertex", "fragment")
                .unwrap();

        Self {
            gl,
            vao,
            vbo,
            ebo,
            program,
            texture_array_id,
        }
    }

    // pub fn set_active_piece(&self, active_piece: Option<(u32, u32)>) {

    // }

    pub fn render(&self, bb: &BitBoard) {
        unsafe {
            self.program.use_program();
            self.gl
                .bind_texture(glow::TEXTURE_2D_ARRAY, Some(self.texture_array_id));
            self.gl.bind_vertex_array(Some(self.vao));

            for row in 0..8 {
                for col in 0..8 {
                    if let Some(piece) = bb.get_piece_at(row * 8 + col) {
                        // position
                        self.gl.uniform_2_f32(
                            self.program.get_uniform_location("piece_position").as_ref(),
                            col as _,
                            row as _,
                        );

                        // texture index
                        self.gl.uniform_1_i32(
                            self.program.get_uniform_location("texture_index").as_ref(),
                            piece.get_texture_index() as _,
                        );

                        self.gl
                            .draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
                    }
                }
            }
        }
    }

    fn load_texture(gl: &'a glow::Context) -> glow::NativeTexture {
        print!("[Info] Loading textures\n       [ ");

        const IMG_WIDTH: u32 = 2000;
        const IMG_HEIGHT: u32 = 668;

        const ROWS: u32 = 2;
        const COLS: u32 = 6;

        const PIECE_WIDTH: f32 = IMG_WIDTH as f32 / COLS as f32;
        const PIECE_HEIGHT: f32 = IMG_HEIGHT as f32 / ROWS as f32;

        let img = image::open("assets/pieces.png").unwrap().into_rgba8();

        unsafe {
            let id = gl.create_texture().unwrap();

            gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(id));
            gl.tex_storage_3d(
                glow::TEXTURE_2D_ARRAY,
                1,
                glow::RGBA8,
                PIECE_WIDTH as _,
                PIECE_HEIGHT as _,
                12,
            );

            let mut z_offset = 0;
            for row in 0..ROWS {
                for col in 0..COLS {
                    // rotate 180° and flip horizontally
                    let sub_img_data =
                        image::imageops::flip_horizontal(&image::imageops::rotate180(
                            &img.view(
                                (col as f32 * PIECE_WIDTH) as _,
                                (row as f32 * PIECE_HEIGHT) as _,
                                PIECE_WIDTH as _,
                                PIECE_HEIGHT as _,
                            )
                            .to_image(),
                        ));

                    gl.tex_sub_image_3d(
                        glow::TEXTURE_2D_ARRAY,
                        0,
                        0,
                        0,
                        z_offset,
                        PIECE_WIDTH as _,
                        PIECE_HEIGHT as _,
                        1,
                        glow::RGBA,
                        glow::UNSIGNED_BYTE,
                        glow::PixelUnpackData::Slice(Some(sub_img_data.as_raw())),
                    );

                    print!("{z_offset} ");
                    std::io::stdout().flush().unwrap();
                    z_offset += 1;
                }
            }

            println!("]");

            const TARGET: u32 = glow::TEXTURE_2D_ARRAY;
            gl.generate_mipmap(TARGET);
            gl.tex_parameter_i32(TARGET, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(TARGET, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(TARGET, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(TARGET, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

            println!("       Done");

            id
        }
    }
}

impl Drop for PieceRenderer<'_> {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_buffer(self.ebo);
            self.gl.delete_texture(self.texture_array_id);
        }
    }
}

fn create_quad_mesh(
    gl: &glow::Context,
) -> (
    glow::NativeVertexArray,
    glow::NativeBuffer,
    glow::NativeBuffer,
) {
    // Create a simple quad for rendering both the board and pieces
    #[rustfmt::skip]
    const VERTICES: [f32; 16] = [
        // positions   // texture coords
        -0.5,  0.5,    0.0, 1.0,  // top left
        -0.5, -0.5,    0.0, 0.0,  // bottom left
         0.5, -0.5,    1.0, 0.0,  // bottom right
         0.5,  0.5,    1.0, 1.0,  // top right
    ];

    const INDICES: [u32; 6] = [
        0, 1, 2, // first triangle
        0, 2, 3, // second triangle
    ];

    let vao = unsafe { gl.create_vertex_array().unwrap() };
    let vbo = unsafe { gl.create_buffer().unwrap() };
    let ebo = unsafe { gl.create_buffer().unwrap() };

    unsafe {
        gl.bind_vertex_array(Some(vao));

        // VBO data
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&VERTICES),
            glow::STATIC_DRAW,
        );

        // EBO data
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&INDICES),
            glow::STATIC_DRAW,
        );

        // Position attribute
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            0,                                     // location
            2,                                     // size (2 floats per vertex for position)
            glow::FLOAT,                           // type
            false,                                 // normalized
            4 * std::mem::size_of::<f32>() as i32, // stride
            0,                                     // offset
        );

        // Texture coord attribute
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(
            1,                                     // location
            2,                                     // size (2 floats per vertex for texture coords)
            glow::FLOAT,                           // type
            false,                                 // normalized
            4 * std::mem::size_of::<f32>() as i32, // stride
            2 * std::mem::size_of::<f32>() as i32, // offset
        );

        gl.bind_vertex_array(None);
    }

    (vao, vbo, ebo)
}
