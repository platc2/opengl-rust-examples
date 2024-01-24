#version 430 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;


void main() {

    fragColor = vec3(1., 1., 1.);
}
