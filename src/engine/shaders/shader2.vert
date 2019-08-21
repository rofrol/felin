#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec3 in_color;

layout(location = 0) out vec3 frag_color;

layout(set = 0, binding = 0) uniform Globals {
    mat4 proj;
    mat4 view;
};

layout(set = 0, binding = 1) uniform Locals {
    mat4 transform;
};

void main() {
    gl_Position = proj * view * transform * vec4(in_position, 0.0, 1.0);
    frag_color = in_color;
}