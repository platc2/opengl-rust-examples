#version 330 core

layout(location = 0) in vec3 vertex_color;

out vec4 frag_color;

void main() {
    frag_color = vec4(vertex_color, 1.);
}
