#version 430 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec2 in_uv;

layout (location = 0) out SHADER_VARYING {
    vec2 uv;
} OUT;

layout (std140, binding = 0) uniform Matrices {
    mat4 u_modelViewProjection;
};


void main()
{
    gl_Position = u_modelViewProjection * vec4(in_position, 1.0);
    OUT.uv = in_uv;
}
