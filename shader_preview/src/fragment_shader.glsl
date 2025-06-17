#version 330 core
uniform float u_time;
uniform float u_gamma;
uniform vec3 u_color;
out vec4 FragColor;

void main() {
    FragColor = vec4(u_color, u_gamma);
}
