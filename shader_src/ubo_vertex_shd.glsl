#version 330 core

layout (location=0) in vec3 in_Position;

uniform mat4 model_mat;
layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};

void main() {
    gl_Position = transpose(projection) * transpose(view) * model_mat * vec4(in_Position, 1.0);
}
