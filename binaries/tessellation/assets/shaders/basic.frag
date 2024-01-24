#version 440 core


layout (location = 0) out vec4 color;

layout (std140, binding = 0) uniform Data {
    float gamma;
};

void main(void) {
    color = vec4(vec3(1), 1);
//    color.rgb = pow(color.rgb, vec3(1 / gamma));
}
