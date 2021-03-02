#version 330 core

layout (points) in;
layout (triangle_strip, max_vertices = 5) out;

in VS_Out {
    vec3 color;
    vec2 tex_coords;
    vec3 normal;
    vec3 world_pos;
} gs_in[];
out vec3 Color;

void build_house(vec4 position);

void main() {
    build_house(gl_in[0].gl_Position);
}

void build_house(vec4 position) {
    Color = gs_in[0].color;
    gl_Position = position + vec4(-0.2, -0.2, 0.0, 0.0);
    EmitVertex();
    gl_Position = position + vec4(0.2, -0.2, 0.0, 0.0);
    EmitVertex();
    gl_Position = position + vec4(-0.2, 0.2, 0.0, 0.0);
    EmitVertex();
    gl_Position = position + vec4(0.2, 0.2, 0.0, 0.0);
    EmitVertex();
    gl_Position = position + vec4(0.0, 0.4, 0.0, 0.0);
    Color = vec3(1.0);
    EmitVertex();
    EndPrimitive();
}