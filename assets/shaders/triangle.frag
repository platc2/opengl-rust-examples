#version 430 core

layout (location = 0) in SHADER_VARYING {
    vec2 uv;
} IN;
layout (binding = 0) uniform sampler2D tex;

layout (location = 0) out vec4 color;


void main()
{
    color = vec4(1.0, 1.0, 1.0, 1.0);
    color = vec4(IN.uv, 0.0, 1.0);

    color = texture(tex, IN.uv);
}
