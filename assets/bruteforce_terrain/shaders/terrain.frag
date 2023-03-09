#version 410 core
#extension GL_ARB_shading_language_420pack : require

layout (location = 0) in SHADER_VARYING {
    vec3 normal;
    vec2 uv;
} pass;

layout (binding = 0) uniform sampler2D heightmap;

layout (location = 0) out vec4 color;


void main() {
    const vec2 uv = pass.uv;
    const float offset = 1.0 / 256.0;
    vec3 normal = vec3(0.0, offset * 2.0, 0.0);
    normal.x = texture(heightmap, uv + vec2(0.0, offset)).r - texture(heightmap, uv - vec2(0.0, offset)).r;
    normal.z = texture(heightmap, uv + vec2(offset, 0.0)).r - texture(heightmap, uv - vec2(offset, 0.0)).r;
    normal = normalize(normal);
    const float diffuse = dot(normal, normalize(vec3(0.5, 1.0, 0.5)));
    color.rgb = vec3(1.0) * diffuse;
    color.rgb = 1.2 - exp(-color.rgb);
}
