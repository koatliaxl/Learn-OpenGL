#version 330 core

layout (location=0) in vec3 in_Position;
layout (location=2) in vec2 in_Tex_Coords;
layout (location=4) in mat4 ia_Model_Mat;
layout (location=8) in vec3 ia_offset;

out vec2 Tex_Coords;

// "Some drivers do not implement uniform initializers correctly" https://www.khronos.org/opengl/wiki/Uniform_(GLSL)
uniform int draw_option = 1;
uniform mat4 model_mat;
layout (std140) uniform Matrices {
    mat4 view;
    mat4 projection;
};

void main() {
    switch (draw_option) {
        case 1:
            gl_Position = projection * view * ia_Model_Mat * vec4(in_Position, 1.0);
            break;
        case 2:
            gl_Position = projection * view * model_mat * vec4(in_Position, 1.0);
            break;
        case 3:
            float scale = (gl_InstanceID + 1) / 100.0;
            gl_Position = projection * view * vec4(in_Position * scale + ia_offset, 1.0);
            break;
        case 4:
            /*mat4 instance_mat = model_mat;
            instance_mat[3][0] = 1.5 * gl_InstanceID;
            gl_Position = projection * view * instance_mat * vec4(in_Position, 1.0);*/
            /*vec3 shift = vec3(1.5 * gl_InstanceID, 0.0, 0.0);
            gl_Position = projection * view * vec4(in_Position + shift, 1.0);*/
            break;
    }
    Tex_Coords = in_Tex_Coords;
}