#version 330 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in VS_Out {
    vec3 color;
    vec2 tex_coords;
    vec3 normal;
    vec3 world_pos;
} gs_in[];

out vec2 Tex_Coords;

uniform float time;

vec3 compute_normal(vec3 p1, vec3 p2, vec3 p3);
vec4 explode(vec4 position, vec3 normal);

void main() {
    vec3 normal = compute_normal(vec3(gl_in[0].gl_Position), vec3(gl_in[1].gl_Position), vec3(gl_in[2].gl_Position));

    gl_Position = explode(gl_in[0].gl_Position, normal);
    Tex_Coords = gs_in[0].tex_coords;
    EmitVertex();

    gl_Position = explode(gl_in[1].gl_Position, normal);
    Tex_Coords = gs_in[1].tex_coords;
    EmitVertex();

    gl_Position = explode(gl_in[2].gl_Position, normal);
    Tex_Coords = gs_in[2].tex_coords;
    EmitVertex();

    EndPrimitive();
}

vec3 compute_normal(vec3 p1, vec3 p2, vec3 p3) {
    vec3 a = p1 - p2;
    vec3 b = p3 - p2;
    return normalize(cross(a, b));
}

const float magnidude = 2.0;
vec4 explode(vec4 position, vec3 normal) {
    vec3 direction = normal * ((sin(time) + 1.0) / 2.0) * magnidude;
    return position + vec4(direction, 0.0);
}
