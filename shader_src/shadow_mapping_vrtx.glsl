#version 330 core

layout (location=0) in vec3 in_Position;
layout (location=1) in vec3 in_Normal;
layout (location=2) in vec2 in_Tex_Coords;
layout (location=3) in vec3 in_Color;

out vec3 Color;
out vec2 Tex_Coords;
out vec3 Normal;
out vec3 World_Pos;
out vec4 Light_Space_Pos;

uniform mat4 model_mat;
layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};
uniform mat4 light_space_matrix;

void main() {
    vec4 world_pos = model_mat * vec4(in_Position, 1.0);
    gl_Position = projection * view * world_pos;

    mat3 normal_matrix = transpose(inverse(mat3(model_mat)));
    Normal = normalize(normal_matrix * in_Normal);
    Tex_Coords = in_Tex_Coords;
    Color = in_Color;
    World_Pos = world_pos.xyz;
    Light_Space_Pos = light_space_matrix * world_pos;
}
