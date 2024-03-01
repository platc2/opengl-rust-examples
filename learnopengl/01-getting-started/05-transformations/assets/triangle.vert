#version 330 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 tex_coord;

layout(location = 0) out vec3 vertex_color;
layout(location = 1) out vec2 vertex_tex_coord;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(pos, 1.);
    vertex_color = color;
    vertex_tex_coord = tex_coord;
}
