#version 450


layout(location = 0) in vec3 v_WorldPosition;
layout(location = 1) in vec3 v_WorldNormal;
layout(location = 2) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform MyMaterial_albedo_color {
    vec4 color;
};

void main() {
    o_Target = color;
}
