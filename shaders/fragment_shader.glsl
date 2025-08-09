#version 460 core
out vec4 FragColor;
in vec3 pos;
void main() {
    FragColor = vec4(abs(sin(pos.x)), abs(sin(pos.y)), 0.2, 1.0);
}