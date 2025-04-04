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
        vec4(8.0, 8.0, 8.0, 1.0),
        vec4(0.2, 0.2, 0.2, 1.0)
    };

uniform ivec2 active_piece;

void main() {
    int x = int(floor(frag_coord.x / STEP));
    int y = int(floor(frag_coord.y / STEP));

    // base color
    if (x == active_piece.x && y == active_piece.y) {
        frag_color = vec4(0.1, 0.6, 0.1, 1.0);
    } else {
        frag_color = COLORS[(x + y) % 2];
    }
}
