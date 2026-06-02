#version 450 core

layout (location = 0) in vec3 vertex;

layout (std140, binding = 0) uniform Matrix {
    mat4 model;
    mat4 view;
    mat4 projection;
} matrix;


void main() {
    vec4 modelSpace = normalize(matrix.model * vec4(vertex, 1.));
    mat4 viewProjection = matrix.projection * matrix.view;

//    mat4 modelViewProjection = matrix.projection * matrix.view * matrix.model;
//    gl_Position = modelViewProjection * vec4(vertex, 1.);
    gl_Position = viewProjection * modelSpace;
}
