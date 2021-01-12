#version 330 core

in vec3 Normal;
in vec3 Frag_pos;
in vec2 Tex_coords;

out vec4 FragColor;

//uniform vec3 viewer_position;

struct Material {
    sampler2D texture;
    sampler2D specular_map;
    float shininess;
    sampler2D emission_map;
};
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
struct SpotLight {
    PointLight point_light;
    vec3 direction;
    float inner_cutoff;
    float outer_cutoff;
};
struct DirectionalLight {
    vec3 direction;
    Light light;
};

uniform Material material;
uniform Light default_light;
uniform Attenuation default_attenuation;
uniform PointLight light;
#define POINT_LIGHTS_NUM 4
uniform PointLight point_lights[POINT_LIGHTS_NUM];
uniform SpotLight flash_light;
uniform vec3 world_up;
uniform float time_mod;

vec3 calc_point_light(PointLight pl, vec3 normal, vec3 viewer_dir);
vec3 calc_spotlight(SpotLight sl, vec3 normal, vec3 viewer_dir);
vec3 calc_directional_light(DirectionalLight dl, vec3 normal, vec3 viewer_dir);

void main() {
    vec3 norm = normalize(Normal);
    vec3 viewer_dir = normalize(-Frag_pos);
    //vec3 viewer_dir = normalize(viewer_position - Frag_pos);

    vec3 total_lighting = vec3(0.0);

    PointLight cube_light = light;
    cube_light.light = default_light;
    cube_light.attenuation = default_attenuation;
    total_lighting += calc_point_light(cube_light, norm, viewer_dir);

    for (int i = 0; i < POINT_LIGHTS_NUM; i++) {
        PointLight pl = point_lights[i];
        pl.light.diffuse = pl.light.specular * 0.5;
        pl.attenuation = default_attenuation;
        total_lighting += calc_point_light(pl, norm, viewer_dir);
    }

    SpotLight fl = flash_light;
    fl.point_light.position = vec3(0.0);
    fl.direction = vec3(0.0, 0.0, -1.0);
    fl.point_light.attenuation = default_attenuation;
    fl.point_light.light.diffuse =  fl.point_light.light.specular * 0.5;
    total_lighting += calc_spotlight(fl, norm, viewer_dir);

    DirectionalLight sun;
    sun.direction = world_up;
    sun.light = default_light;
    total_lighting += calc_directional_light(sun, norm, viewer_dir);

    float ds = 0.15;
    vec2 emission_coords = Tex_coords * (1.0 + ds * 2.0) - ds;
    vec3 glow = vec3(texture(material.emission_map, emission_coords));

    FragColor = vec4(total_lighting + glow, 1.0);
}

vec3 ambient_lighting(vec3 light);
vec3 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal);
vec3 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
float attenuation(vec3 source_pos, Attenuation attenuation);

vec3 calc_point_light(PointLight pl, vec3 normal, vec3 viewer_dir) {
    vec3 result = vec3(0.0);
    vec3 light_dir = normalize(pl.position - Frag_pos);
    result += ambient_lighting(pl.light.ambient);
    result += diffuse_lighting(pl.light.diffuse, light_dir, normal);
    result += specular_lighting(pl.light.specular, light_dir, normal, viewer_dir);
    result *= attenuation(pl.position, pl.attenuation);
    return result;
}

vec3 calc_spotlight(SpotLight sl, vec3 normal, vec3 viewer_dir) {
    vec3 result = vec3(0.0);
    PointLight pl = sl.point_light;
    vec3 light_dir = normalize(pl.position - Frag_pos);
    float theta = dot(light_dir, normalize(-sl.direction));
    if (theta > sl.outer_cutoff) {
        result += diffuse_lighting(pl.light.diffuse, light_dir, normal);
        result += specular_lighting(pl.light.specular, light_dir, normal, viewer_dir);
        if (theta < sl.inner_cutoff) {
            float intensity = (theta - sl.outer_cutoff) / (sl.inner_cutoff - sl.outer_cutoff);
            result *= intensity;
        }
    }
    result += ambient_lighting(pl.light.ambient);
    result *= attenuation(pl.position, pl.attenuation);
    return result;
}

vec3 calc_directional_light(DirectionalLight dl, vec3 normal, vec3 viewer_dir) {
    vec3 result = vec3(0.0);
    vec3 light_dir = normalize(dl.direction);
    result += ambient_lighting(dl.light.ambient);
    result += diffuse_lighting(dl.light.diffuse, light_dir, normal);
    result += specular_lighting(dl.light.specular, light_dir, normal, viewer_dir);
    return result;
}

vec3 ambient_lighting(vec3 light) {
    return light * vec3(texture(material.texture, Tex_coords));
}

vec3 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal) {
    float diffuse_impact = max(dot(normal, light_dir), 0.0);
    return light * diffuse_impact * vec3(texture(material.texture, Tex_coords));
}

vec3 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir) {
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular_impact = pow(max(dot(viewer_dir, reflect_dir), 0.0), material.shininess);
    return light * specular_impact * vec3(texture(material.specular_map, Tex_coords));
}

float attenuation(vec3 source_pos, Attenuation attenuation) {
    float distance = length(source_pos - Frag_pos);
    return 1.0 / (attenuation.constant_term + attenuation.linear_term * distance
        + attenuation.quadratic_term * distance * distance);
}