#version 330 core

in vec4 Frag_Pos;
uniform vec3 Light_Pos;
uniform float Far_Plane;

void main() {
    // get distance between fragment and light source
    float light_distance = length(Frag_Pos.xyz - Light_Pos);
    // map to [0;1] range by dividing by far_plane
    light_distance /= Far_Plane;
    // write this as modified depth
    gl_FragDepth = light_distance;
}
