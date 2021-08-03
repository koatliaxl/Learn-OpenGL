#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 18) out;

in VS_Out {
    vec3 color;
    vec2 tex_coords;
    vec3 normal;
    vec3 world_pos;
} gs_in[];
uniform mat4 light_space_matrices[6];
out vec4 Frag_Pos; // FragPos from GS (output per emited vertex)

void main() {
    for (int face = 0; face < 6; ++face) {
        gl_Layer = face; // built-in variable that specifies to which face we render
        for (int i = 0; i < 3; ++i) // for each triangle vertex
        {
            Frag_Pos = vec4(gs_in[i].world_pos, 1.0);
            gl_Position = light_space_matrices[face] * Frag_Pos;
            EmitVertex();
        }
        EndPrimitive();
    }
}
