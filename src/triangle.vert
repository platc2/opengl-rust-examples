#version 430 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

out VS_OUTPUT {
    vec3 color;
} OUT;


void main()
{
    gl_Position = vec4(position, 1.0);
    OUT.color = color;
}
