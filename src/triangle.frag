#version 430 core

in VS_OUTPUT {
    vec3 color;
} IN;

out vec4 color;


void main()
{
    color = vec4(IN.color, 1.0);
}
