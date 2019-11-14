#version 450

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec4 in_color;
layout(location = 2) in vec2 in_tex_coord;


layout(set = 0, binding = 0) uniform Globals { mat4 ortho; };
layout(set = 0, binding = 1) uniform Locals { mat4 transform; };

layout(location = 0) out vec2 v_tex_coord;

void main() {
    v_tex_coord = in_tex_coord;
    gl_Position = ortho * transform * vec4(in_position, 0.0, 1.0);
}