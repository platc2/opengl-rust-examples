#version 330 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 tex_coord;

layout(location = 0) out vec3 vertex_color;
layout(location = 1) out vec2 vertex_tex_coord;

uniform vec2 offset;

void main() {
    gl_Position = vec4(pos.xy + offset, pos.z, 1.);
    vertex_color = color;
    vertex_tex_coord = tex_coord;
}
