#version 330 core

struct Light {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};
struct Attenuation {
    float constant_term;
    float linear_term;
    float quadratic_term;
};
struct PointLight {
    vec3 position;
    Light light;
    Attenuation attenuation;
};

/*struct Material {
    sampler2D diffuse_texture;
    sampler2D specular_map;
    float shininess;
    //sampler2D emission_map;
};*/

in vec3 World_Pos;
in vec3 Normal;
in vec2 Tex_Coords;

out vec4 FragColor;

uniform sampler2D diffuse_texture;
uniform sampler2D specular_map;
uniform float shininess;
uniform PointLight point_lights[1];
uniform vec3 viewer_position;

vec4 calc_point_light(PointLight pl, vec3 normal, vec3 viewer_dir);

void main() {
    vec4 total_color = vec4(0.0);
    vec3 viewer_dir = normalize(viewer_position - World_Pos);
    vec3 norm = normalize(Normal);

    Light default_light = Light(vec3(0.2), vec3(0.5), vec3(1.0));
    //Attenuation default_attenuation = Attenuation(1.0, 0.09, 0.032);
    Attenuation default_attenuation = Attenuation(1.0, 0.0, 0.0);

    for (int i = 0; i < 1; i++) {
        PointLight pl = point_lights[i];
        pl.light = default_light;
        pl.attenuation = default_attenuation;
        total_color += calc_point_light(pl, norm, viewer_dir);
    }

    FragColor = total_color;
}

vec4 ambient_lighting(vec3 light);
vec4 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal);
vec4 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
float attenuation(vec3 source_pos, Attenuation attenuation);

vec4 calc_point_light(PointLight pl, vec3 normal, vec3 viewer_dir) {
    vec4 result = vec4(0.0);
    vec3 light_dir = normalize(pl.position - World_Pos);
    result += ambient_lighting(pl.light.ambient);
    result += diffuse_lighting(pl.light.diffuse, light_dir, normal);
    result += specular_lighting(pl.light.specular, light_dir, normal, viewer_dir);
    result *= attenuation(pl.position, pl.attenuation);
    return result;
}

vec4 ambient_lighting(vec3 light) {
    return vec4(light, 1.0) * vec4(texture(diffuse_texture, Tex_Coords));
}

vec4 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal) {
    float diffuse_impact = max(dot(normal, light_dir), 0.0);
    return vec4(light, 1.0) * diffuse_impact * vec4(texture(diffuse_texture, Tex_Coords));
}

vec4 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir) {
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular_impact = pow(max(dot(viewer_dir, reflect_dir), 0.0), shininess);
    return vec4(light, 1.0) * specular_impact * vec4(texture(specular_map, Tex_Coords));
}

float attenuation(vec3 source_pos, Attenuation attenuation) {
    float distance = length(source_pos - World_Pos);
    return 1.0 / (attenuation.constant_term + attenuation.linear_term * distance
        + attenuation.quadratic_term * distance * distance);
}