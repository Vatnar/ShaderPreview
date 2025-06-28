#version 330 core

layout (location = 0) in vec2 aPos;
uniform float u_time;

uniform vec2 u_offset;
// TODO offset by zoom somehow, scaling
uniform float u_zoom;

void main() {
    // zoom center is at screen center (0,0) — no need to compute zoom center in this case
    vec2 zoomedPos = (aPos + u_offset) * u_zoom;

    gl_Position = vec4(zoomedPos, 0.0, 1.0);
    gl_PointSize = 20.0;
}


