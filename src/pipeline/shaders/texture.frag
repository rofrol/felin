#version 450

layout(location = 0) in vec2 v_tex_coord;
layout(location = 0) out vec4 o_target;

//Texture data
layout(set = 1, binding = 0) uniform texture2D in_texture;
layout(set = 1, binding = 1) uniform sampler s_color;

void main() {
    vec4 texel = texture(sampler2D(in_texture, s_color), v_tex_coord);

    o_target = texel;
}