#version 450

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec3 in_color;

//Instance data
layout(location = 2) in vec2 translation;
layout(location = 3) in vec2 scale;
layout(location = 4) in vec4 instance_color;


//Bindings
layout(set = 0, binding = 0) uniform Globals {
    mat4 ortho;
};

layout(set = 1, binding = 0) uniform Model {
    mat4 transform_buffer;
};

//To frag shader
layout(location = 0) out vec3 frag_color;

mat4 instance_transform = mat4(
    vec4(scale.x, 0.0, 0.0, 0.0),
    vec4(0.0, scale.y, 0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(translation, 0.0, 1.0)
);

void main() {
    gl_Position = ortho * transform_buffer * instance_transform * vec4(in_position, 0.0, 1.0);
    frag_color = in_color;
}