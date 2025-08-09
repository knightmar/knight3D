#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor; // the color variable has attribute position 1

out vec3 pos;
out vec3 color;
void main() {
    gl_Position = vec4(aPos, 1.0);
    pos = aPos;
    color = aColor;
}