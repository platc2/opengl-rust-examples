#version 410 core
#extension GL_ARB_shading_language_420pack : require


layout (location = 0) out vec4 color;

layout (std140, binding = 0) uniform Data {
    float gamma;
};

void main(void) {
    color = vec4(vec3(1), 1);
//    color.rgb = pow(color.rgb, vec3(1 / gamma));
}
