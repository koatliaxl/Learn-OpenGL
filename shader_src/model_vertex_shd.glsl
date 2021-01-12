#version 330 core

layout (location=0) in vec3 in_Position;
layout (location=1) in vec3 in_Normal;
layout (location=2) in vec2 in_Tex_Coords;

out vec3 World_Pos;
out vec3 Normal;
out vec2 Tex_Coords;

uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 projection_mat;

void main() {
    vec4 world_pos = model_mat * vec4(in_Position, 1.0);
    gl_Position = projection_mat * view_mat * world_pos;
    Normal = mat3(transpose(inverse(model_mat))) * in_Normal;
    Tex_Coords = in_Tex_Coords;
    World_Pos = world_pos.xyz;
}
