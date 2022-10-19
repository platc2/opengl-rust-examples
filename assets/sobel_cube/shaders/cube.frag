#version 430 core

layout (location = 0) in SHADER_VARYING {
    vec2 uv;
} IN;

layout (std140, binding = 0) uniform Kernel {
    mat4 kernel;
};

layout (binding = 0) uniform sampler2D tex;

layout (location = 0) out vec4 color;


void main()
{
    vec2 offset = 1.0 / vec2(900, 700);
    vec2 offsets[9] = vec2[](
        vec2(-offset.x,  offset.y), // top-left
        vec2( 0.0f,      offset.y), // top-center
        vec2( offset.x,  offset.y), // top-right
        vec2(-offset.x,  0.0f),     // center-left
        vec2( 0.0f,      0.0f),     // center-center
        vec2( offset.x,  0.0f),     // center-right
        vec2(-offset.x, -offset.y), // bottom-left
        vec2( 0.0f,     -offset.y), // bottom-center
        vec2( offset.x, -offset.y)  // bottom-right
    );

    vec3 sampleTex[9];
    for(int i = 0; i < 9; ++i)
    {
        sampleTex[i] = vec3(texture(tex, IN.uv + offsets[i]));
    }

    float filterKernel[] = float[9](
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
            2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0
    );

    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; ++i)
    {
        int filterKernelX = i % 3;
        int filterKernelY = i / 3;
        col += sampleTex[i] * kernel[filterKernelY][filterKernelX];
    }

    color = vec4(col, 1.0);
}
