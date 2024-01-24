#version 410 core
#extension GL_ARB_shading_language_420pack : require

layout (location = 0) in vec3 vertex;

layout (location = 0) out struct {
    vec2 uv;
} vertex_out;

layout (binding = 0) uniform sampler2D heightmap;

layout (std140, binding = 0) uniform Matrix {
    mat4 model;
    mat4 view;
    mat4 projection;
} matrix;


vec3 offset_vertex(vec3 vertex, float height);

void main() {
    const vec2 uv = vec2((vertex.x + 1.0) / 2.0, (vertex.z + 1.0) / 2.0);

    float height = texture(heightmap, uv).r;
    vec3 vertex = offset_vertex(vertex, height);
    mat4 modelViewProjection = matrix.projection * matrix.view * matrix.model;
    gl_Position = modelViewProjection * vec4(vertex, 1.0);
    vertex_out.uv = uv;
}

vec3 offset_vertex(vec3 vertex, float height) {
    vertex.y += height;
    return vertex;
}
