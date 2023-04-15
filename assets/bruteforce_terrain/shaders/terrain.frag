#version 410 core
#extension GL_ARB_shading_language_420pack: require

layout (location = 0) in vec2 uv;

layout (binding = 0) uniform sampler2D heightmap;

layout (binding = 1) uniform sampler2D grass_texture;
layout (binding = 2) uniform sampler2D sand_texture;
layout (binding = 3) uniform sampler2D stone_texture;
layout (binding = 4) uniform sampler2D snow_texture;

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
ColorGradient(0.6875, vec3(0.675, 0.875, 0.0)),
ColorGradient(0.8, vec3(0.5, 0.5, 0.5)),
ColorGradient(1.0, vec3(1.0, 1.0, 1.0))
};
const float num_color_gradients = color_gradients.length();

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
vec3 compute_normal(vec2 uv);
vec3 compute_color(float height);

vec3 light_direction = normalize(vec3(-0.5, 1.0, 0.5));


void main() {
    vec3 normal = compute_normal(uv);
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

vec3 compute_normal(vec2 uv) {
    const float texel_size = 1.0 / 64.0;

    float u = texture(heightmap, uv + texel_size * vec2(0.0, -1.0)).r;
    float r = texture(heightmap, uv + texel_size * vec2(1.0, 0.0)).r;
    float l = texture(heightmap, uv + texel_size * vec2(-1.0, 0.0)).r;
    float d = texture(heightmap, uv + texel_size * vec2(0.0, 1.0)).r;
    vec3 normal = vec3(0.0, 2.0 * texel_size, 0.0);
    normal.x = l - r;
    normal.z = u - d;
//    vec3 normal = vec3(0.0, offset * 2.0, 0.0);
//    normal.x = texture(heightmap, uv + vec2(0.0, offset)).r - texture(heightmap, uv - vec2(0.0, offset)).r;
//    normal.z = texture(heightmap, uv + vec2(offset, 0.0)).r - texture(heightmap, uv - vec2(offset, 0.0)).r;
//    normal = normalize(normal);
    return normalize(normal);
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
