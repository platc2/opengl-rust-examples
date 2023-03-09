#version 410 core
#extension GL_ARB_shading_language_420pack : require


layout (vertices = 3) out;

layout (std140, binding = 0) uniform TessellationParameters {
    uint outer[4];
    uint inner[2];
} tessellation_parameters;


void main() {
    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

    if (gl_InvocationID == 0) {
        gl_TessLevelOuter[0] = tessellation_parameters.outer[0];
        gl_TessLevelOuter[1] = tessellation_parameters.outer[1];
        gl_TessLevelOuter[2] = tessellation_parameters.outer[2];
        gl_TessLevelOuter[3] = tessellation_parameters.outer[3];

        gl_TessLevelInner[0] = tessellation_parameters.inner[0];
        gl_TessLevelInner[1] = tessellation_parameters.inner[1];
    }
}
