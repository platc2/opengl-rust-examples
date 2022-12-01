#version 420 core

layout (location = 0) in SHADER_VARYING {
    vec3 normal;
    vec2 uv;
} pass;

layout (binding = 0) uniform sampler2D heightmap;

layout (location = 0) out vec4 color;


void main() {
/*
    const float height = pass.color.y;
    color = vec4(vec3(height), 1.0);

    color.rgb = vec3(0.0, 0.1, 0.5);
    if (height > -0.03) { color.rgb = vec3(0.0, 0.2, 0.6); }
    if (height > -0.02) { color.rgb = vec3(0.0, 0.2, 0.8); }
    if (height > 0.0) { color.rgb = vec3(0.5, 0.8, 0.2); }
    if (height > 0.1) { color.rgb = vec3(0.0, 0.4, 0.0); }
    if (height > 0.2) { color.rgb = vec3(0.0, 0.2, 0.0); }
    if (height > 0.3) { color.rgb = vec3(0.4); }
    if (height > 0.4) { color.rgb = vec3(0.5); }
    if (height > 0.5) { color.rgb = vec3(1.0); }
*/

    color.rgb = 1.2 - exp(-color.rgb);

    const vec2 uv = pass.uv;
    const float offset = 1.0 / 16.0;
    vec3 normal = vec3(0.0, offset * 2.0, 0.0);
    normal.x = texture(heightmap, uv + vec2(0.0, offset)).r - texture(heightmap, uv - vec2(0.0, offset)).r;
    normal.z = texture(heightmap, uv + vec2(offset, 0.0)).r - texture(heightmap, uv - vec2(offset, 0.0)).r;
    normal = normalize(normal);
    const float diffuse = dot(normal, vec3(0.0, 1.0, 0.0));
    color.rgb = vec3(1.0) * diffuse;
    color.rgb = normal;
    color.rgb = vec3(texture(heightmap, uv).r);
}
