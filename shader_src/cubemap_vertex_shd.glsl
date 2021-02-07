#version 330 core

layout (location=0) in vec3 in_Position;

out vec3 Cubemap_Tex_Coords;

uniform mat4 view_mat;
uniform mat4 projection_mat;

void main() {
    Cubemap_Tex_Coords = in_Position;
    vec4 pos = projection_mat * view_mat * vec4(in_Position, 1.0);
    gl_Position = pos.xyww;
}
