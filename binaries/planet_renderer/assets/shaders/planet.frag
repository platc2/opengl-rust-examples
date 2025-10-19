#version 450 core

layout (location = 0) out vec3 fragColor;

layout (location = 0) uniform bool s = false;


void main() {
    if (s) {
        fragColor = vec3(1., 0., 0.);
    } else {
        fragColor = vec3(1.);
    }
}
