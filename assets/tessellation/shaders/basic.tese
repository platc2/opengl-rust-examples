#version 430

// layout (triangles, fractional_odd_spacing, ccw) in;
layout (triangles, equal_spacing, ccw) in;


void main() {
    const vec4 a = gl_in[0].gl_Position;
    const vec4 b = gl_in[1].gl_Position;
    const vec4 c = gl_in[2].gl_Position;

    const vec4 a_u = a + ((b - a) + (c - a)) / 2;
    const vec4 b_v = b + ((a - b) + (c - b)) / 2;
    const vec4 c_w = c + ((a - c) + (b - c)) / 2;

    gl_Position = a_u * gl_TessCoord.x + b_v * gl_TessCoord.y + c_w * gl_TessCoord.z;

/*
    const vec4 d = b - a;
    const vec4 e = c - a;

    gl_Position = a + (gl_TessCoord.x * d) + (gl_TessCoord.y * e);
*/
}
