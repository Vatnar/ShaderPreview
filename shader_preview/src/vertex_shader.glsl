#version 330 core

layout (location = 0) in vec2 aPos;
uniform float u_time;

void main() {
    float offsetX = 0.1 * sin(5.0 * aPos.y + u_time * 3.0);
    float offsetY = 0.1 * cos(5.0 * aPos.x + u_time * 3.0);

    vec2 funkyPos = aPos + vec2(offsetX, offsetY);

    gl_Position = vec4(funkyPos, 0.0, 1.0);
}
