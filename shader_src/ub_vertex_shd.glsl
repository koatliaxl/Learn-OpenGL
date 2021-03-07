#version 330 core

layout (location=0) in vec3 in_Position;
layout (location=1) in vec3 in_Normal;
layout (location=2) in vec2 in_Tex_Coords;
layout (location=3) in vec3 in_Color;

out vec3 Color;
out vec2 Tex_Coords;

uniform mat4 model_mat;
layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};

void main() {
    vec4 world_pos = model_mat * vec4(in_Position, 1.0);
    gl_Position = transpose(projection) * transpose(view) * world_pos;

    Tex_Coords = in_Tex_Coords;
    Color = in_Color;
}
