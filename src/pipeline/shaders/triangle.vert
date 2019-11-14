#version 450

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec4 in_color;


layout(set = 0, binding = 0) uniform Globals { mat4 ortho; };
layout(set = 0, binding = 1) uniform Locals { mat4 transform; };

layout(location = 0) out vec4 frag_color;

void main() {
    frag_color = in_color;
    gl_Position = ortho * transform * vec4(in_position, 0.0, 1.0);
}