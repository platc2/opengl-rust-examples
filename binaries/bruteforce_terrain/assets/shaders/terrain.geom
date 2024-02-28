#version 450 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

layout (location = 0) in struct {
    vec2 uv;
} geometry_in[];

layout (location = 0) out struct {
    vec2 uv;
    vec3 normal;
} geometry_out;

layout (binding = 0) uniform sampler2D heightmap;


vec3 compute_normal(vec2 uv) {
    const float scalingFactor = 1.0 / 32.0;
    const vec2 offset = vec2(1, 0) * scalingFactor;
    const float hL = texture(heightmap, uv - offset.xy).r;
    const float hR = texture(heightmap, uv + offset.xy).r;
    const float hD = texture(heightmap, uv - offset.yx).r;
    const float hU = texture(heightmap, uv + offset.yx).r;

    return normalize(vec3(
    hL - hR,
    2.0 * scalingFactor,
    hD - hU));
}


void main() {
    for (int i = 0; i < 3; ++i) {
        gl_Position = gl_in[i].gl_Position;
        geometry_out.normal = compute_normal(geometry_in[i].uv);
        geometry_out.uv = geometry_in[i].uv;
        EmitVertex();
    }

    EndPrimitive();
}
