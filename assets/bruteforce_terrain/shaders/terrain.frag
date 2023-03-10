#version 410 core
#extension GL_ARB_shading_language_420pack : require

layout (location = 0) in SHADER_VARYING {
    vec3 normal;
    vec2 uv;
} pass;

layout (binding = 0) uniform sampler2D heightmap;

layout (location = 0) out vec4 color;


struct ColorGradient {
    float value;
    vec3 color;
};

const ColorGradient color_gradients[] = {
ColorGradient(0.0, vec3(0.0, 0.0, 0.5)),
ColorGradient(0.375, vec3(0, 0.0, 1.0)),
ColorGradient(0.5, vec3(0.0, 0.5, 1.0)),
ColorGradient(0.53125, vec3(0.9375, 0.9375, 0.25)),
ColorGradient(0.5625, vec3(0.125, 0.625, 0.0)),
ColorGradient(0.6875, vec3(0.875, 0.875, 0.0)),
ColorGradient(0.875, vec3(0.5, 0.5, 0.5)),
ColorGradient(1.0, vec3(1.0, 1.0, 1.0))
};
const float num_color_gradients = color_gradients.length();


vec3 compute_normal(vec2 uv);

vec3 compute_color(float height);


void main() {
    const vec2 uv = pass.uv;
    vec3 normal = compute_normal(uv);
    // Compute diffuse shading
    const float diffuse = dot(normal, normalize(vec3(0.5, 1.0, 0.5)));
    float height = texture(heightmap, uv).r;
    color.rgb = compute_color(height) * diffuse;
}

vec3 compute_normal(vec2 uv) {
    const float offset = 1.0 / 256.0;
    vec3 normal = vec3(0.0, offset * 2.0, 0.0);
    normal.x = texture(heightmap, uv + vec2(0.0, offset)).r - texture(heightmap, uv - vec2(0.0, offset)).r;
    normal.z = texture(heightmap, uv + vec2(offset, 0.0)).r - texture(heightmap, uv - vec2(offset, 0.0)).r;
    normal = normalize(normal);
    return normal;
}

vec3 compute_color(float height) {
    for (int i = 0; i < num_color_gradients; ++i) {
        ColorGradient current_color_gradient = color_gradients[i];
        if (height <= current_color_gradient.value) {
            if (i == 0) {
                return current_color_gradient.color;
            } else {
                ColorGradient previous_color_gradient = color_gradients[i - 1];
                float len = current_color_gradient.value - previous_color_gradient.value;
                float t = (height - previous_color_gradient.value) / len;
                return mix(previous_color_gradient.color, current_color_gradient.color, t);
            }
        }
    }
    return vec3(0.0);
}
