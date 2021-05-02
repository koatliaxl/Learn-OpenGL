#version 330 core

//in vec2 Tex_Coords;
out vec4 Frag_Color;
uniform sampler2DMS screen_texture;

//vec3 clear_color = vec3(0.0);
vec3 color_threshold = vec3(0.0, 0.8, 0.0);
int samples = 4;

void main() {
    ivec2 tex_coords = ivec2(gl_FragCoord.x, gl_FragCoord.y);
    int count = 0;
    vec3 color = vec3(0.0);
    for (int i; i < samples; i++) {
        vec3 subsample = vec3(texelFetch(screen_texture, tex_coords, i));
        color += subsample;
        if (subsample.r >= color_threshold.r &&
            subsample.g >= color_threshold.g &&
            subsample.b >= color_threshold.b) {
            count++;
        }
    }
    switch (count) {
        case 1: Frag_Color = vec4(0.0, 0.0, 1.0, 1.0);
            return;
        case 2: Frag_Color = vec4(1.0, 1.0, 0.0, 1.0);
            return;
        case 3: Frag_Color = vec4(1.0, 0.0, 1.0, 1.0);
            return;
    }
    Frag_Color = vec4(color / samples, 1.0);
}
