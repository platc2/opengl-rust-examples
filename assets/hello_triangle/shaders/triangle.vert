#version 410

layout (location = 0) in vec2 vertex;
layout (location = 1) in vec3 color;

layout (location = 0) out SHADER_VARYING {
    vec3 fragment_color;
};

void main(void) {
    gl_Position = vec4(vertex, 0, 1);
    fragment_color = color;
}
