#version 450

layout(location = 0) in vec2 uv_cords;
layout(location = 1) flat in uint layer_index;

layout(set = 1, binding = 0) uniform texture2DArray in_texture;
layout(set = 1, binding = 1) uniform sampler s_color;

layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = texture(sampler2DArray(in_texture, s_color), vec3(uv_cords, layer_index));
}





