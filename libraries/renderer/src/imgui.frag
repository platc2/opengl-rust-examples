#version 410 core
#extension GL_ARB_shading_language_420pack : require

in vec2 Frag_UV;
in vec4 Frag_Color;

layout (binding = 1) uniform sampler2D Texture;

layout (location = 0) out vec4 Out_Color;


void main() {
    Out_Color = Frag_Color * texture(Texture, Frag_UV.st);
}