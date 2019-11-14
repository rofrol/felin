#version 450

//Vector position
layout(location = 0) in vec2 in_position;
layout(location = 1) in vec4 in_color;

//Texture stuff
layout(location = 2) in vec2 in_tex_coord;
layout(location = 3) in uint in_texture_index;


layout(set = 0, binding = 0) uniform Globals { mat4 ortho; };
layout(set = 0, binding = 1) uniform Locals { mat4 transform; };

layout(location = 0) out vec2 out_tex_coord;
layout(location = 1) out uint out_texture_index;

void main() {
    out_tex_coord = in_tex_coord;
    out_texture_index = in_texture_index;

    gl_Position = ortho * transform * vec4(in_position, 0.0, 1.0);
}