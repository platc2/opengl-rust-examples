#version 330 core

layout(location = 0) in vec2 vertex_tex_coord;

out vec4 frag_color;

uniform sampler2D texture1;
uniform sampler2D texture2;

uniform float texture_factor;
uniform bool flip_face;
uniform float texture_scale;

void main() {
    vec2 scaled_tex_coord = vertex_tex_coord / texture_scale;
    frag_color = mix(
        texture(texture1, scaled_tex_coord),
        texture(texture2, scaled_tex_coord + vec2(flip_face ? 1. : 0., 0.)),
        texture_factor
    );
}
