#version 430 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec2 in_uv;
layout (location = 2) in vec3 in_normal;

layout (std140, binding = 0) uniform Matrices {
    mat4 u_model;
    mat4 u_viewProjection;
};

layout (location = 0) out SHADER_VARYING {
vec2 uv;
vec3 normal;
vec3 frag_coord;
} OUT;


void main()
{
    gl_Position = u_viewProjection * u_model * vec4(in_position, 1.0);
    OUT.uv = in_uv;
    OUT.normal = mat3(transpose(inverse(u_model))) * in_normal;
    OUT.frag_coord = vec3(u_model * vec4(in_position, 1.0));
}
