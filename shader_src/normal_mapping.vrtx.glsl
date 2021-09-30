#version 330 core

layout (location=0) in vec3 in_Position;
layout (location=1) in vec3 in_Normal;
layout (location=2) in vec2 in_Tex_Coords;
//layout (location=3) in vec3 in_Color;
layout (location=4) in vec3 in_Tangent;
layout (location=5) in vec3 in_Bitangent;

uniform mat4 model_mat;
layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};

uniform vec3[10] light_positions;
uniform int Light_Sources_Num = 0;
uniform vec3 Viewer_Position;
uniform bool bitangent_generation = true;
uniform bool re_orthonormalize_tangents = false;

out vec3[10] TangentSpace_LightPositions;
out vec3 TangentSpace_ViewerPos;
out vec3 TangentSpace_FragPos;

out vec2 Tex_Coords;

void main() {
    vec4 world_pos = model_mat * vec4(in_Position, 1.0);
    gl_Position = projection * view * world_pos;
    Tex_Coords = in_Tex_Coords;

    mat3 normal_matrix = transpose(inverse(mat3(model_mat)));
    vec3 T = normalize(normal_matrix * in_Tangent);
    vec3 N = normalize(normal_matrix * in_Normal);
    if (re_orthonormalize_tangents) {
        T = normalize(T - dot(T, N) * N);
    }
    vec3 B;
    if (bitangent_generation) {
        B = cross(N, T);
        //B = cross(T, N);
    } else {
        B = normalize(normal_matrix * in_Bitangent);
    }
    mat3 TBN_matrix = transpose(mat3(T, B, N));

    TangentSpace_ViewerPos = TBN_matrix * Viewer_Position;
    TangentSpace_FragPos = TBN_matrix * world_pos.xyz;
    for (int i = 0; i < Light_Sources_Num; i++) {
        TangentSpace_LightPositions[i] = TBN_matrix * light_positions[i];
    }
}
