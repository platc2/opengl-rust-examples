#version 430 core

layout (location = 0) in SHADER_VARYING {
    vec2 uv;
    vec3 normal;
    vec3 frag_coord;
} IN;
layout (binding = 0) uniform sampler2D t_cube;
layout (binding = 1) uniform sampler2D t_floor;
layout (std140, binding = 1) uniform Settings {
    float tex_switch;
};

layout (location = 0) out vec3 color;

const vec3 LIGHT_POSITION = vec3(5, 0, 2.5);
const vec3 LIGHT_COLOR = vec3(0.6, 0.6, 1);
const vec3 AMBIENT_COLOR = vec3(0.1, 0.1, 0.1);

void main()
{
    vec3 norm = normalize(IN.normal);
    vec3 lightDir = normalize(LIGHT_POSITION - IN.frag_coord);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * LIGHT_COLOR;
    vec3 light = AMBIENT_COLOR + diffuse;

    vec3 object_color = mix(
        texture(t_cube, IN.uv).rgb,
        texture(t_floor, IN.uv).rgb,
        tex_switch
    );
    color = object_color * light;
}
