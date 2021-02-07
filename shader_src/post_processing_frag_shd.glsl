#version 330 core

in vec2 Tex_Coords;

out vec4 FragColor;

uniform sampler2D Screen_Texture;
uniform int mode;

const float sharpen_kernel[9] = float[](
    -1, -1, -1,
    -1,  9, -1,
    -1, -1, -1
);
const float sharpen_kernel_2[9] = float[](
     0, -1,  0,
    -1,  5, -1,
     0, -1,  0
);
const float box_blur[9] = float[] (
    1, 1, 1,
    1, 1, 1,
    1, 1, 1
); // divide by 9
const float gaussian_blur_3x3[9] = float[](
    1, 2, 1,
    2, 4, 2,
    1, 2, 1
); // divide by 16
const float edge_detection_kernel[9] = float[](
    1,  1,  1,
    1, -8,  1,
    1,  1,  1
);
const float emboss_kernel[9] = float[] (
    -2, -1,  0,
    -1,  1,  1,
     0,  1,  2
);
const float[9] custom_kernel_2 = float[] (
     2,   2,   2,
     2, -14,   2,
     2,   2,   2
); // div by 2
const float[9] vertical_edge_detection = float[] (
    -1,  2, -1,
    -1,  2, -1,
    -1,  2, -1
);
const float gaussian_blur_5x5[25] = float[] (
    1,  4,  6,  4,  1,
    4, 16, 24, 16,  4,
    6, 24, 36, 24,  6,
    4, 16, 24, 16,  4,
    1,  4,  6,  4,  1
); // divide by 256
const float[25] custom_kernel_1 = float[] (
     1,  1,  1,  1,  1,
     1, -2, -2, -2,  1,
     1, -2,  6, -2,  1,
     1, -2, -2, -2,  1,
     1,  1,  1,  1,  1
);

vec3 kernel(float[9] kernel);
vec3 kernel_5x5(float[25] kernel);

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
            FragColor = vec4(kernel(gaussian_blur_3x3) / 16.0, 1.0);
            break;
        case 6:
            FragColor = vec4(kernel(edge_detection_kernel), 1.0);
            break;
        case 7:
            FragColor = vec4(kernel_5x5(gaussian_blur_5x5) / 256.0, 1.0);
            break;
        case 8:
            FragColor = vec4(kernel(emboss_kernel), 1.0);
            break;
        case 9:
            FragColor = vec4(kernel(box_blur) / 9.0, 1.0);
            break;
        case 10:
            FragColor = vec4(kernel(sharpen_kernel_2), 1.0);
            break;
        case 11:
            FragColor = vec4(kernel_5x5(custom_kernel_1), 1.0);
            break;
        case 12:
            FragColor = vec4(kernel(custom_kernel_2) / 2.0, 1.0);
            break;
        case 13:
            FragColor = vec4(kernel(vertical_edge_detection), 1.0);
            break;
    }
}

const float offset = 1.0 / 300.0;
const vec2 offsets[9] = vec2[](
    vec2(-offset, offset), // top-left
    vec2( 0.0,    offset), // top-center
    vec2( offset, offset), // top-right
    vec2(-offset, 0.0),   // center-left
    vec2( 0.0,    0.0),   // center-center
    vec2( offset, 0.0),   // center-right
    vec2(-offset, -offset), // bottom-left
    vec2( 0.0,    -offset), // bottom-center
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

const float offset_2 = offset * 2.0;
const vec2 offsets_5x5[25] = vec2[](
    vec2(-offset_2, offset_2),
    vec2(-offset,   offset_2),
    vec2( 0.0,      offset_2),
    vec2( offset,   offset_2),
    vec2( offset_2, offset_2),

    vec2(-offset_2, offset),
    vec2(-offset,   offset),
    vec2( 0.0,      offset),
    vec2( offset,   offset),
    vec2( offset_2, offset),

    vec2(-offset_2, 0.0),
    vec2(-offset,   0.0),
    vec2( 0.0,      0.0),
    vec2( offset,   0.0),
    vec2( offset_2, 0.0),

    vec2(-offset_2, -offset),
    vec2(-offset,   -offset),
    vec2( 0.0,      -offset),
    vec2( offset,   -offset),
    vec2( offset_2, -offset),

    vec2(-offset_2, -offset_2),
    vec2(-offset,   -offset_2),
    vec2( 0.0,      -offset_2),
    vec2( offset,   -offset_2),
    vec2( offset_2, -offset_2)
);

vec3 kernel_5x5(float[25] kernel) {
    vec3 color = vec3(0.0);
    for (int i = 0; i < 25; i++) {
        vec3 samle = vec3(texture(Screen_Texture, Tex_Coords + offsets_5x5[i]));
        color += samle * kernel[i];
    }
    return color;
}