#version 330 core

layout (location=0) in vec3 in_Position;
layout (location=1) in vec3 in_Normal;
layout (location=2) in vec2 in_Tex_Coords;
layout (location=3) in vec3 in_Color;

out vec3 Color;
out vec2 Tex_Coords;
out vec3 Normal;
out vec3 World_Pos;

uniform mat4 model_mat;
layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};
uniform bool reverse_normals = false;

void main() {
    vec4 world_pos = model_mat * vec4(in_Position, 1.0);
    gl_Position = projection * view * world_pos;

    mat3 normal_matrix = mat3(transpose(inverse(model_mat)));
    Normal = normalize(normal_matrix * in_Normal);
    if (reverse_normals) {
        // a slight hack to make sure the outer large cube can display lighting from the 'inside'
        Normal *= -1.0;
    }
    Tex_Coords = in_Tex_Coords;
    Color = in_Color;
    World_Pos = world_pos.xyz;
}
