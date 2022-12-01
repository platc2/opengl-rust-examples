#version 420 core

layout (location = 0) in vec3 vertex;

layout (location = 0) out SHADER_VARYING {
    vec3 normal;
    vec2 uv;
} pass;

layout (binding = 0) uniform sampler2D heightmap;

layout (std140, binding = 0) uniform Matrix {
    mat4 model;
    mat4 view;
    mat4 projection;
} matrix;



void main() {
    const vec2 uv = vec2((vertex.x + 1.0) / 2.0, (vertex.z + 1.0) / 2.0);
    const float height = texture(heightmap, uv).r;
    pass.uv = uv;
    const vec3 heightmap_vertex = vec3(vertex.x, vertex.y + height * 0.2, vertex.z);

    // compute normal vector
    const float offset = 1.0 / 256.0;
    vec3 normal = vec3(0.0, offset * 10.0, 0.0);
    normal.x = texture(heightmap, uv + vec2(0.0, offset)).r - texture(heightmap, uv - vec2(0.0, offset)).r;
    normal.z = texture(heightmap, uv + vec2(offset, 0.0)).r - texture(heightmap, uv - vec2(offset, 0.0)).r;
    normal = normalize(normal);

    gl_Position = matrix.projection * matrix.view * matrix.model * vec4(heightmap_vertex, 1.0);
    pass.normal = normal;
}
