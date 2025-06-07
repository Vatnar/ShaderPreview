#version 330 core
uniform float u_time;
out vec4 FragColor;

void main() {
    float green = 0.5 + 0.5 * sin(u_time * 5);
    FragColor = vec4(1.0, green, 0.2, 1.0);
}
