#version 450 core

layout (location = 0) in vec3 vertex;

layout (std140, binding = 0) uniform Matrix {
    mat4 model;
    mat4 view;
    mat4 projection;
} matrix;


void main() {
    mat4 modelViewProjection = matrix.projection * matrix.view * matrix.model;
    gl_Position = modelViewProjection * vec4(vertex, 1.0);
}
