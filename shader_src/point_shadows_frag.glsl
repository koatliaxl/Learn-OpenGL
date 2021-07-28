#version 330 core
out Frag_Color;

in vec3 World_Pos;
in vec3 Normal;
in vec2 Tex_Coords;

uniform samplerCube Depth_Cubemap;
uniform sampler2D Diffuse_Texture;
uniform vec3 Viewer_Pos;
uniform float Far_Plane;

struct LightSource {
    vec3 position;
    vec3 color;
};
uniform LightSource Light_Source;

uniform float Shininess;
uniform float attenuation_constant_term = 1.0;
uniform float attenuation_linear_term = 0.0;
uniform float attenuation_quadratic_term = 0.0;
uniform float ambient_strength = 0.2;
uniform float specular_coef = 0.3;

vec4 calc_point_light(LightSource pl, vec3 normal, vec3 viewer_dir, vec3 frag_pos);
float calc_shadow_value(vec4 frag_pos, vec3 ligh_pos);

void main() {
    vec3 viewer_dir = normalize(Viewer_Position - World_Pos);
    for (int i = 0; i < Light_Sources_Num; i++) {
        Frag_Color += calc_point_light(Light_Sources[i], Normal, viewer_dir, World_Pos);
    }
}

float calc_shadow_value(vec3 frag_pos, vec3 ligh_pos) {
    // get vector between fragment position and light position
    vec3 frag_to_light = frag_pos - ligh_pos;
    float closest_depth = texture(Depth_Cubemap, frag_to_light.r);
    // re-transform back from range [0, 1] to [0, far_plane]
    closest_depth *= Far_Plane;
    float frag_depth = lengh(frag_to_light);
    // test for shadows
    float bias = 0.05;
    return frag_depth - bias > closest_depth ? 1.0 : 0.0;
}

vec4 ambient_lighting(vec3 light);
vec4 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal);
vec4 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
vec4 blinn_phong_specular(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
float attenuation(vec3 source_pos, vec3 frag_pos);

vec4 calc_point_light(LightSource pl, vec3 normal, vec3 viewer_dir, vec3 frag_pos) {
    vec3 light_dir = normalize(pl.position - frag_pos);
    vec4 ambient = ambient_lighting(pl.color);
    vec4 diffuse = diffuse_lighting(pl.color, light_dir, normal);
    vec4 specular = blinn_phong_specular(pl.color, light_dir, normal, viewer_dir) * specular_coef;

    float shadow_value = calc_shadow_value(frag_pos, pl.position);
    vec4 lighting = ambient + (1.0 - shadow_value) * (diffuse + specular);

    lighting *= attenuation(pl.position, frag_pos);
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

float attenuation(vec3 source_pos, vec3 frag_pos) {
    float distance = length(source_pos - frag_pos);
    return 1.0 / (attenuation_constant_term
        + attenuation_linear_term * distance
        + attenuation_quadratic_term * distance * distance);
}
