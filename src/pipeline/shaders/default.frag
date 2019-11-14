#version 450

layout(location = 0) in vec2 uv_cords;
layout(location = 1) in vec4 in_color;
layout(location = 2) flat in int layer_index;

layout(set = 1, binding = 0) uniform texture2DArray in_texture;
layout(set = 1, binding = 1) uniform sampler s_color;

layout(location = 0) out vec4 o_target;

void main() {
    if (layer_index >= 0) {
        o_target = texture(sampler2DArray(in_texture, s_color), vec3(uv_cords, layer_index));
    } else {
        o_target = in_color;
    }
}





