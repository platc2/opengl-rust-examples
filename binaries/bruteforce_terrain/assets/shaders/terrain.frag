#version 450 core

layout (location = 0) in struct {
    vec2 uv;
    vec3 normal;
} fragment_in;

layout (binding = 0) uniform sampler2D heightmap;

layout (binding = 1) uniform sampler2D grass_texture;
layout (binding = 2) uniform sampler2D sand_texture;
layout (binding = 3) uniform sampler2D stone_texture;
layout (binding = 4) uniform sampler2D snow_texture;

layout (location = 0) out vec4 color;


struct TextureGradient {
    float value;
    int index;
};

const TextureGradient texture_gradients[] = {
TextureGradient(0.0, 0),
TextureGradient(0.4, 0),
TextureGradient(0.6, 1),
TextureGradient(0.875, 2),
TextureGradient(0.9, 3),
TextureGradient(1.0, 3),
};
const float num_texture_gradients = texture_gradients.length();


vec3 compute_texture(float height, vec2 uv);
vec3 light_direction = normalize(vec3(-0.5, 1.0, 0.5));


void main() {
    const vec2 uv = fragment_in.uv;
    const vec3 normal = fragment_in.normal;

    // Compute diffuse shading
    float diffuse = max(dot(normal, light_direction), 0.0);
    const float ambient = 0.0;
    float light = min(1.0, diffuse + ambient);

    float height = texture(heightmap, uv).r;
    float steepness = 1.0 - abs(dot(normal, vec3(0.0, 1.0, 0.0)));
    vec3 computed_texture = compute_texture(height, mod(uv * 128, 1));
    if (steepness >= 0.8) {
        color.rgb = texture(stone_texture, mod(uv * 128, 1)).rgb;
    } else if (steepness >= 0.7) {
        float t = (0.8 - steepness) * 10;
        color.rgb = mix(texture(stone_texture, mod(uv * 128, 1)).rgb, computed_texture, t);
    } else {
        color.rgb = computed_texture;
    }

    color.rgb *= light;
}

vec3 get_texture(int index, vec2 uv) {
    switch (index) {
        case 0: return texture(sand_texture, uv).rgb;
        case 1: return texture(grass_texture, uv).rgb;
        case 2: return texture(stone_texture, uv).rgb;
        case 3: return texture(snow_texture, uv).rgb;
        default : return vec3(1.0, 0.0, 0.0);
    }
}

vec3 compute_texture(float height, vec2 uv) {
    for (int i = 0; i < num_texture_gradients; ++i) {
        TextureGradient current_texture_gradient = texture_gradients[i];
        if (height <= current_texture_gradient.value) {
            if (i == 0) {
                return get_texture(current_texture_gradient.index, uv);
            } else {
                TextureGradient previous_texture_gradient = texture_gradients[i - 1];
                float len = current_texture_gradient.value - previous_texture_gradient.value;
                float t = (height - previous_texture_gradient.value) / len;
                return mix(
                get_texture(previous_texture_gradient.index, uv),
                get_texture(current_texture_gradient.index, uv),
                t);
            }
        }
    }
    return vec3(1.0, 0.0, 0.0);
}
