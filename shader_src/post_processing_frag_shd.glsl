#version 330 core

in vec2 Tex_Coords;

out vec4 FragColor;

uniform sampler2D Screen_Texture;
uniform int mode;

float sharpen_kernel[9] = float[](
    -1, -1, -1,
    -1,  9, -1,
    -1, -1, -1
);
float blur_kernel[9] = float[](
    1, 2, 1,
    2, 4, 2,
    1, 2, 1
); // divide by 16
float edge_detection_kernel[9] = float[](
    1,  1,  1,
    1, -8,  1,
    1,  1,  1
);

vec3 kernel(float[9] kernel);

void main() {
    switch (mode) {
        case 0:
            FragColor = texture(Screen_Texture, Tex_Coords);
            break;
        case 1:
            FragColor = vec4(vec3(1.0 - texture(Screen_Texture, Tex_Coords)), 1.0);
            break;
        case 2:
            FragColor = texture(Screen_Texture, Tex_Coords);
            float average = (FragColor.r + FragColor.g + FragColor.b) / 3.0;
            FragColor = vec4(vec3(average), 1.0);
            break;
        case 3:
            FragColor = texture(Screen_Texture, Tex_Coords);
            average = FragColor.r * 0.2126 + FragColor.g * 0.7152 + FragColor.b * 0.0722;
            FragColor = vec4(vec3(average), 1.0);
            break;
        case 4:
            FragColor = vec4(kernel(sharpen_kernel), 1.0);
            break;
        case 5:
            FragColor = vec4(kernel(blur_kernel) / 16.0, 1.0);
            break;
        case 6:
            FragColor = vec4(kernel(edge_detection_kernel), 1.0);
            break;
    }
}

const float offset = 1.0 / 300.0;
const vec2 offsets[9] = vec2[](
    vec2(-offset,  offset), // top-left
    vec2( 0.0f,    offset), // top-center
    vec2( offset,  offset), // top-right
    vec2(-offset,  0.0f),   // center-left
    vec2( 0.0f,    0.0f),   // center-center
    vec2( offset,  0.0f),   // center-right
    vec2(-offset, -offset), // bottom-left
    vec2( 0.0f,   -offset), // bottom-center
    vec2( offset, -offset)  // bottom-right
);

vec3 kernel(float[9] kernel) {
    vec3 color = vec3(0.0);
    for (int i = 0; i < 9; i++) {
        vec3 samle = vec3(texture(Screen_Texture, Tex_Coords + offsets[i]));
        color += samle * kernel[i];
    }
    return color;
}