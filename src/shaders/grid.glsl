-- vertex
#version 420 core

layout(location = 0) in vec2 vert_pos;

out vec2 frag_coord;

void main() {
    gl_Position = vec4(vert_pos, 0.0, 1.0);
    frag_coord = (vert_pos + vec2(1.0)) / 2.0;
}

-- fragment
#version 420 core

in vec2 frag_coord;
out vec4 frag_color;

const float STEP = 1.0 / 8.0;
const vec4 COLORS[] = {
    vec4(0.8, 0.8, 0.8, 1.0), // WHITE
    vec4(0.2, 0.2, 0.2, 1.0), // BLACK
};

const vec4 ACTIVE_PIECE_COLOR = vec4(0.1, 0.6, 0.1, 1.0);
const vec4 MOVABLE_PIECE_COLOR = vec4(0.0, 0.4, 0.4, 1.0);

uniform ivec2 active_piece;

// x: ms part, y: ls part
uniform uvec2 piece_moves;

void main() {
    int col = int(frag_coord.x / STEP);
    int row = int(frag_coord.y / STEP);

    int square = row * 8 + col;

    if (
        (square <= 32 && (piece_moves.y & (1 << square)) != 0)
        || (square > 32 && (piece_moves.x & (1 << (square - 32))) != 0)
    ) {
        frag_color = MOVABLE_PIECE_COLOR;
    } else if (col == active_piece.x && row == active_piece.y) {
        frag_color = ACTIVE_PIECE_COLOR;
    } else {
        frag_color = COLORS[(row + col) % 2];
    }
}
