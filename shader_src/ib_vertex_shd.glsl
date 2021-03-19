#version 330 core

layout (location=0) in vec3 in_Position;
layout (location=1) in vec3 in_Normal;
layout (location=2) in vec2 in_Tex_Coords;
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
    vec4 world_pos = model_mat * vec4(in_Position, 1.0);
    gl_Position = projection * view * world_pos;

    vs_out.color = in_Color;
    vs_out.tex_coords = in_Tex_Coords;
    mat3 normal_matrix = mat3(transpose(inverse(model_mat)));
    vs_out.normal = normalize(normal_matrix * in_Normal);
    vs_out.world_pos = world_pos.xyz;
}
