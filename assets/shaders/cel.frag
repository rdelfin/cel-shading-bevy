#version 450

const int MAX_LIGHTS = 10;

layout(location = 0) in vec3 v_WorldPosition;
layout(location = 1) in vec3 v_WorldNormal;
layout(location = 2) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(std140, set = 0, binding = 1) uniform CameraPosition {
    vec4 CameraPos;
};

layout(set = 2, binding = 0) uniform MyMaterial_albedo_color {
    vec4 albedo_color;
};

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(std140, set = 3, binding = 0) uniform Lights {
    vec4 AmbientColor;
    // x is the number of lights
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

void main() {
    vec3 light_accum = vec3(0.0);
    for (int i = 0; i < int(1) && i < MAX_LIGHTS; i++) {
        Light light = SceneLights[0];
        vec3 view_dir = normalize(light.pos.xyz - v_WorldPosition);
        float factor = pow(max(-0.0, dot(view_dir, v_WorldNormal)), 1.7);
        light_accum += vec3(factor * albedo_color.xyz);
    }

    o_Target = vec4(light_accum, albedo_color.w);
}
