#version 330 core

in vec2 Tex_Coords;

out vec4 FragColor;

uniform sampler2D texture_0;
uniform int depth_visualization_mode;

float far = 100.0;
float near = 0.1;

float linearize_depth(float depth) {
    float z = depth * 2.0 - 1.0; // back to NDC
    return (2.0 * near * far) / (far + near - z * (far - near));
}

void main() {
    switch (depth_visualization_mode) {
        case 0:
            FragColor = texture(texture_0, Tex_Coords);
            break;
        case 1:
            float depth = linearize_depth(gl_FragCoord.z) / far; // divide by far for demonstration
            FragColor = vec4(vec3(depth), 1.0);
            break;
        case 2:
            FragColor = vec4(vec3(gl_FragCoord.z), 1.0);
            break;
    }
}
