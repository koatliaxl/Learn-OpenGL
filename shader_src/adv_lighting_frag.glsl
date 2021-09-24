#version 330 core

in vec2 Tex_Coords;
in vec3 Normal;
in vec3 World_Pos;
out vec4 Frag_Color;

in vec3[10] TangentSpace_LightPositions;
in vec3 TangentSpace_ViewerPos;
in vec3 TangentSpace_FragPos;

uniform sampler2D Diffuse_Texture;
uniform sampler2D normal_map;
uniform vec3 Viewer_Position;
uniform bool Blinn_Phong_Lighting = true;
uniform bool Gamma_Correction = false;
uniform bool normal_mapping = false;

//uniform bool tangent_space_correction = true;

struct PointLight {
    vec3 position;
    vec3 color;
};
struct DirectionalLight {
    vec3 direction;
    vec3 color;
};
uniform PointLight[10] Light_Sources;
uniform int Light_Sources_Num = 0;
uniform DirectionalLight[3] Dir_Light_Sourses;
uniform int Directional_Light_Num = 0;

uniform float Shininess = 64.0;
uniform float attenuation_constant_term = 1.0;
uniform float attenuation_linear_term = 0.0;
uniform float attenuation_quadratic_term = 0.0;
uniform float ambient_strength = 0.2;
uniform float specular_factor = 0.3;

const float gamma = 2.2;

vec4 calc_point_light(PointLight pl, vec3 normal, vec3 viewer_dir, vec3 frag_pos);
vec4 calc_directional_light(DirectionalLight dl, vec3 normal, vec3 viewer_dir);

void main() {
    vec3 viewer_dir = normalize(Viewer_Position - World_Pos);
    for (int i = 0; i < Light_Sources_Num; i++) {
        if (!normal_mapping) {
            Frag_Color += calc_point_light(Light_Sources[i], Normal, viewer_dir, World_Pos);
        } else {
            vec3 frag_normal = vec3(texture(normal_map, Tex_Coords));
            frag_normal = normalize(frag_normal * 2.0 - 1.0); // transform normal vector to range [-1,1]
            PointLight ls = PointLight(TangentSpace_LightPositions[i], Light_Sources[i].color);
            vec3 tan_space_view_dir = normalize(TangentSpace_ViewerPos - TangentSpace_FragPos);
            Frag_Color += calc_point_light(ls, frag_normal, tan_space_view_dir, TangentSpace_FragPos);
        }
    }
    if (Gamma_Correction) {
        Frag_Color.rgb = pow(Frag_Color.rgb, vec3(1.0 / gamma));
    }
}

vec4 ambient_lighting(vec3 light);
vec4 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal);
vec4 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
vec4 blinn_phong_specular(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir);
float attenuation(vec3 source_pos, vec3 frag_pos);

vec4 calc_point_light(PointLight pl, vec3 normal, vec3 viewer_dir, vec3 frag_pos) {
    vec4 result = vec4(0.0);
    vec3 light_dir = normalize(pl.position - frag_pos);
    result += ambient_lighting(pl.color);
    result += diffuse_lighting(pl.color, light_dir, normal);
    if (Blinn_Phong_Lighting) {
        result += blinn_phong_specular(pl.color, light_dir, normal, viewer_dir) * specular_factor;
    } else {
        result += specular_lighting(pl.color, light_dir, normal, viewer_dir) * specular_factor;
    }
    result *= attenuation(pl.position, frag_pos);
    return result;
}

vec4 calc_directional_light(DirectionalLight dl, vec3 normal, vec3 viewer_dir) {
    vec4 result = vec4(0.0);
    vec3 light_dir = normalize(dl.direction);
    result += ambient_lighting(dl.color);
    result += diffuse_lighting(dl.color, light_dir, normal);
    if (Blinn_Phong_Lighting) {
        result += blinn_phong_specular(dl.color, light_dir, normal, viewer_dir) * specular_factor;
    } else {
        result += specular_lighting(dl.color, light_dir, normal, viewer_dir) * specular_factor;
    }
    return result;
}

vec4 ambient_lighting(vec3 light) {
    return vec4(light, 1.0) * ambient_strength * texture(Diffuse_Texture, Tex_Coords);
}

vec4 diffuse_lighting(vec3 light, vec3 light_dir, vec3 normal) {
    float diffuse_impact = max(dot(normal, light_dir), 0.0);
    return vec4(light, 1.0) * diffuse_impact * texture(Diffuse_Texture, Tex_Coords);
}

vec4 specular_lighting(vec3 light, vec3 light_dir, vec3 normal, vec3 viewer_dir) {
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular_impact = pow(max(dot(viewer_dir, reflect_dir), 0.0), Shininess);
    return vec4(light, 1.0) * specular_impact/* * vec4(texture(specular_map, Tex_Coords))*/;
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
