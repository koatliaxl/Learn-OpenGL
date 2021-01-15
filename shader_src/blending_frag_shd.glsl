#version 330 core

in vec2 Tex_Coords;

out vec4 FragColor;

uniform sampler2D texture_0;

void main() {
    /*vec4 tex_color = texture(texture_0, Tex_Coords);
    if (tex_color.a < 0.1)
        discard;
    FragColor = tex_color;*/
    FragColor = texture(texture_0, Tex_Coords);
}
