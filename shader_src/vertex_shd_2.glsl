#version 330 core

layout (location=0) in vec3 Pos;
layout (location=1) in vec3 Color;
layout (location=2) in vec2 TexCoord;

out vec3 next_Color;
out vec2 next_TexCoord;

uniform mat4 Transform;
uniform mat4 Model_mat;
uniform mat4 View_mat;
uniform mat4 Projection_mat;

void main() {
    //gl_Position = Transform * vec4(Pos, 0.0, 1.0);
    gl_Position = Projection_mat * View_mat * Model_mat * vec4(Pos, 1.0);
    next_Color = Color;
    next_TexCoord = TexCoord;
}