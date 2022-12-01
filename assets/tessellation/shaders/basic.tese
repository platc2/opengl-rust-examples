#version 430

// layout (triangles, fractional_odd_spacing, ccw) in;
layout (triangles, equal_spacing, ccw) in;


void main() {
    gl_Position = vec4(gl_TessCoord, 1);
}
