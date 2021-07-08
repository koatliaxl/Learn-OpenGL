#version 330 core

in vec2 Tex_Coords;
out vec4 FragColor;
uniform sampler2D depth_map;

float far = 7.5;
float near = 1.0;

// required when using a perspective projection matrix
float linearize_depth(float depth) {
    float z = depth * 2.0 - 1.0; // back to NDC
    return (2.0 * near * far) / (far + near - z * (far - near));
}

void main() {
    float depth_value = texture(depth_map, Tex_Coords).r;
    // FragColor = vec4(vec3(linearize_depth(depth_value) / far), 1.0); // perspective
    FragColor = vec4(vec3(depth_value), 1.0); // orthographic
}
