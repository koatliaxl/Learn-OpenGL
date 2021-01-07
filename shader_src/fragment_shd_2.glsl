#version 330 core

out vec4 FragColor;

in vec3 next_Color;
in vec2 next_TexCoord;

uniform vec4 in_color;
uniform sampler2D Wall_texture;
uniform sampler2D AwF_texture;
uniform sampler2D Con_texture;
uniform float Zoom;
uniform float Mix;

void main() {
    vec4 mix_tex = mix(
        texture(Wall_texture, next_TexCoord / Zoom),
        texture(AwF_texture, vec2(1.0 - next_TexCoord.s, next_TexCoord.t)),
        Mix
    );
    vec4 mix_2 = mix(mix_tex, texture(Con_texture, next_TexCoord / Zoom), 0.5);
    vec4 mix_col = mix(vec4(next_Color, 1.0), in_color, 0.5);
    FragColor = mix(mix_2, mix_col, 0.1);
}
