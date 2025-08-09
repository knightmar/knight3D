#version 460 core
out vec4 FragColor;
in vec3 pos;
in vec3 color;

uniform float time;
void main() {
    FragColor = vec4(color * abs(sin(time * 10 * abs(sin(pos.x * pos.y)))), 1.0);
}