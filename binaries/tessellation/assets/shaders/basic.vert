#version 440 core


layout (location = 0) in vec2 vertex;
layout (location = 1) in vec3 color;


void main(void) {
    gl_Position = vec4(vertex, 0, 1);
}
