#version 450 core

layout (location = 0) in vec2 vertex;

layout (location = 0) out SHADER_VARYING {
    vec3 fragment_color;
};

const vec3 colors[] = vec3[](vec3(1, 0, 0), vec3(0, 1, 0), vec3(0, 0, 1));

void main(void) {
    gl_Position = vec4(vertex, 0, 1);
    fragment_color = colors[gl_VertexID % colors.length()];
}
