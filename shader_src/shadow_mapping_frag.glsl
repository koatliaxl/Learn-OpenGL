#version 330 core

in vec2 Tex_Coords;
in vec3 Normal;
in vec3 World_Pos;
in vec4 Light_Space_Pos;
out vec4 Frag_Color;

uniform sampler2D Diffuse_Texture;
uniform sampler2D Shadow_Map;
uniform float Shininess;
uniform vec3 Viewer_Position;
uniform bool Gamma_Correction = false;

struct LightSource {
    vec3 position;
    vec3 color;
};
uniform LightSource[10] Light_Sources;
uniform int Light_Sources_Num = 0;

uniform float attenuation_constant_term = 1.0;
uniform float attenuation_linear_term = 0.0;
uniform float attenuation_quadratic_term = 0.0;

uniform float ambient_strength = 0.05;
uniform float specular_coef = 0.3;

const float gamma = 2.2;

vec4 calc_point_light(LightSource pl, vec3 normal, vec3 viewer_dir);
float calc_shadow_value(vec4 frag_pos_light_space);

void main() {
    vec3 viewer_dir = normalize(Viewer_Position - World_Pos);
    for (int i = 0; i < Light_Sources_Num; i++) {
        Frag_Color += calc_point_light(Light_Sources[i], Normal, viewer_dir);
    }
    // Gamma Correction:
    if (Gamma_Correction) {
        Frag_Color.rgb = pow(Frag_Color.rgb, vec3(1.0 / gamma));
    }
}

//float calc_shadow_value(vec4 frag_pos_light_space);

vec4 ambient_lighting(vec3 light);
vec4 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal);
vec4 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
vec4 blinn_phong_specular(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
float attenuation(vec3 source_pos);

float calc_shadow_value(vec4 frag_pos_light_space) {
    // perform perspective divide
    vec3 proj_coords = frag_pos_light_space.xyz / frag_pos_light_space.w;
    // transform the NDC coordinates to the range [0,1], because this is the range of the depth value
    proj_coords = proj_coords * 0.5 + 0.5;
    // closest depth from the light's point of view
    float closest_depth = texture(Shadow_Map, proj_coords.xy).r;
    float fragment_depth = proj_coords.z;
    float shadow_value = fragment_depth > closest_depth ? 1.0 : 0.0;
    return shadow_value;
}

vec4 calc_point_light(LightSource pl, vec3 normal, vec3 viewer_dir) {
    vec3 light_dir = normalize(pl.position - World_Pos);
    vec4 ambient = ambient_lighting(pl.color);
    vec4 diffuse = diffuse_lighting(pl.color, light_dir, normal);
    vec4 specular = blinn_phong_specular(pl.color, light_dir, normal, viewer_dir) * specular_coef;

    float shadow_value = calc_shadow_value(Light_Space_Pos);
    vec4 lighting = ambient + (1.0 - shadow_value) * (diffuse + specular);

    lighting *= attenuation(pl.position);
    return lighting;
}

vec4 ambient_lighting(vec3 light) {
    return vec4(light, 1.0) * ambient_strength * texture(Diffuse_Texture, Tex_Coords);
}

vec4 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal) {
    float diffuse_impact = max(dot(normal, light_dir), 0.0);
    return vec4(light, 1.0) * diffuse_impact * texture(Diffuse_Texture, Tex_Coords);
}

vec4 blinn_phong_specular(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir) {
    vec3 halfway_vec = normalize(light_dir + viewer_dir);
    float specular_impact = pow(max(dot(normal, halfway_vec), 0.0), Shininess);
    return vec4(light, 1.0) * specular_impact/* * vec4(texture(specular_map, Tex_Coords))*/;
}

float attenuation(vec3 source_pos) {
    float distance = length(source_pos - World_Pos);
    return 1.0 / (attenuation_constant_term
        + attenuation_linear_term * distance
        + attenuation_quadratic_term * distance * distance);
}
