#version 330 core

layout (location=0) in vec3 in_Position;

layout (location=2) in vec3 in_Tex_Coords;
layout (location=3) in vec3 in_Color;

out VS_Out {
    vec3 color;
    vec2 tex_coords;
    vec3 normal;
    vec3 world_pos;
} vs_out;

uniform mat4 model_mat;
layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};

void main() {
    gl_Position = transpose(projection) * transpose(view) * model_mat * vec4(in_Position, 1.0);
    vs_out.color = in_Color;
}
