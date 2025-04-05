-- vertex
#version 420 core

layout(location = 0) in vec2 i_vertex_position;
layout(location = 1) in vec2 i_texture_coordinate;

out vec2 texture_coordinate;

uniform vec2 piece_position;

void main() {
    vec2 offset = piece_position * 0.25 - vec2(1.0) + 0.125;
    gl_Position = vec4(i_vertex_position * 0.25 + offset, 0.0, 1.0);
    texture_coordinate = i_texture_coordinate;
}


-- fragment
#version 420 core

in vec2 texture_coordinate;
out vec4 frag_color;

uniform int texture_index;
uniform sampler2DArray texture_array;

void main() {
    frag_color = texture(texture_array, vec3(texture_coordinate, texture_index));
}
