#version 330 core

layout (triangles) in;
layout (line_strip, max_vertices = 6) out;

in VS_Out {
    vec3 color;
    vec2 tex_coords;
    vec3 normal;
    vec3 world_pos;
} gs_in[];

out vec2 Tex_Coords;

layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};

void generate_line(int index);

void main() {
    generate_line(0);
    Tex_Coords = gs_in[0].tex_coords;
    generate_line(1);
    Tex_Coords = gs_in[1].tex_coords;
    generate_line(2);
    Tex_Coords = gs_in[2].tex_coords;
}

const float MAGNITUDE = 0.4;

void generate_line(int index) {
    mat4 trp_view = transpose(view);
    mat4 trp_projection = transpose(projection);

    gl_Position = trp_projection * trp_view * vec4(gs_in[index].world_pos, 1.0);
    EmitVertex();

    gl_Position = trp_projection * trp_view * vec4(gs_in[index].world_pos + gs_in[index].normal * MAGNITUDE, 1.0);
    EmitVertex();

    EndPrimitive();
}
