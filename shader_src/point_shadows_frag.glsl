#version 330 core
out vec4 Frag_Color;

in vec3 World_Pos;
in vec3 Normal;
in vec2 Tex_Coords;

uniform samplerCube Depth_Cubemap;
uniform sampler2D Diffuse_Texture;
uniform vec3 Viewer_Position;
uniform float Far_Plane;
uniform float Shadow_Bias;
uniform bool visualize_cubemap_depth_buffer;
uniform bool PCF;
uniform float PCF_Disk_Radius;

struct LightSource {
    vec3 position;
    vec3 color;
};
uniform LightSource Light_Source;

uniform float Shininess = 64.0;
uniform float attenuation_constant_term = 1.0;
uniform float attenuation_linear_term = 0.0;
uniform float attenuation_quadratic_term = 0.0;
uniform float ambient_strength = 0.2;
uniform float specular_coef = 0.3;

vec4 calc_point_light(LightSource pl, vec3 normal, vec3 viewer_dir, vec3 frag_pos);
float closest_depth(vec3 frag_pos, vec3 ligh_pos);

void main() {
    vec3 viewer_dir = normalize(Viewer_Position - World_Pos);
    if (!visualize_cubemap_depth_buffer) {
        Frag_Color = calc_point_light(Light_Source, Normal, viewer_dir, World_Pos);
    } else {
        Frag_Color = vec4(vec3(closest_depth(World_Pos, Light_Source.position)), 1.0);
    }
}

float calc_shadow_value(vec3 frag_pos, vec3 ligh_pos) {
    // get vector between fragment position and light position
    vec3 frag_to_light = frag_pos - ligh_pos;
    float closest_depth = texture(Depth_Cubemap, frag_to_light).r;
    // re-transform back from range [0, 1] to [0, far_plane]
    closest_depth *= Far_Plane;
    float frag_depth = length(frag_to_light);
    // test for shadows
    return frag_depth - Shadow_Bias > closest_depth ? 1.0 : 0.0;
}

const int samples = 20;
const vec3[samples] sample_offset_directions = vec3[] (
    vec3( 1,  1,  1), vec3( 1, -1,  1), vec3(-1, -1,  1), vec3(-1,  1,  1),
    vec3( 1,  1, -1), vec3( 1, -1, -1), vec3(-1, -1, -1), vec3(-1,  1, -1),
    vec3( 1,  1,  0), vec3( 1, -1,  0), vec3(-1, -1,  0), vec3(-1,  1,  0),
    vec3( 1,  0,  1), vec3(-1,  0,  1), vec3( 1,  0, -1), vec3(-1,  0, -1),
    vec3( 0,  1,  1), vec3( 0, -1,  1), vec3( 0, -1, -1), vec3( 0,  1, -1)
);

float pcf_shadow_value(vec3 frag_pos, vec3 ligh_pos) {
    vec3 frag_to_light = frag_pos - ligh_pos;
    float frag_depth = length(frag_to_light);
    float pcf_disk_radius;
    if (PCF_Disk_Radius < 0.0) {
        float view_distance = length(Viewer_Position - frag_pos);
        pcf_disk_radius = (1.0 + (view_distance / Far_Plane)) / 25.0;
    } else {
        pcf_disk_radius = PCF_Disk_Radius;
    }
    float shadow_value = 0.0;
    for (int i = 0; i < samples; ++i) {
        vec3 offset = sample_offset_directions[i] * pcf_disk_radius;
        float closest_depth = texture(Depth_Cubemap, frag_to_light + offset).r;
        closest_depth *= Far_Plane;
        if (frag_depth - Shadow_Bias > closest_depth)
            shadow_value += 1.0;
    }
    return shadow_value / float(samples);
}

float closest_depth(vec3 frag_pos, vec3 ligh_pos) {
    vec3 frag_to_light = frag_pos - ligh_pos;
    float closest_depth = texture(Depth_Cubemap, frag_to_light).r;
    return closest_depth;
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
    float shadow_value;
    if (!PCF) {
        shadow_value = calc_shadow_value(frag_pos, pl.position);
    } else {
        shadow_value = pcf_shadow_value(frag_pos, pl.position);
    }
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
