#version 330 core

in vec3 World_Pos;
in vec3 Normal;
in vec3 Cubemap_Tex_Coords;

out vec4 FragColor;

uniform samplerCube Skybox;
uniform int mode;
uniform vec3 camera_position;

const float air_refrative_index = 1.0;
const float glass_refrative_index = 1.52;

void main() {
    vec3 camera_dir = normalize(World_Pos - camera_position);
    switch (mode) {
        case 0:
            FragColor = texture(Skybox, Cubemap_Tex_Coords);
            break;
        case 1:
            vec3 reflect_dir = reflect(camera_dir, normalize(Normal));
            //FragColor = vec4(texture(Skybox, reflect_dir).rgb, 1.0);
            FragColor = texture(Skybox, reflect_dir);
            break;
        case 2:
            float ratio = air_refrative_index / glass_refrative_index;
            vec3 refract_dir = refract(camera_dir, normalize(Normal), ratio);
            //FragColor = vec4(texture(Skybox, refract_dir).rgb, 1.0);
            FragColor = texture(Skybox, refract_dir);
            break;
    }
}
