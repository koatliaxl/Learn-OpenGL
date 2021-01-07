#version 330 core

layout (location=0) in vec3 Position;
layout (location=1) in vec3 Normal_attrib;
layout (location=2) in vec2 Tex_attrib;

out vec3 Normal;
out vec3 Frag_pos;
out vec2 Tex_coords;

uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 projection_mat;

void main() {
    //vec4 world_pos = model_mat * vec4(Position, 1.0);
    vec4 view_pos = view_mat * model_mat * vec4(Position, 1.0);
    //gl_Position = projection_mat * view_mat * world_pos;
    gl_Position = projection_mat * view_pos;
    //Normal = mat3(transpose(inverse(model_mat))) * Normal_attrib;
    Normal = mat3(transpose(inverse(view_mat * model_mat))) * Normal_attrib;
    //Frag_pos = world_pos.xyz;
    Frag_pos = view_pos.xyz;
    Tex_coords = Tex_attrib;
}
