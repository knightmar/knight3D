#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in mat4 aModel;

uniform float time;
uniform mat4 view;
uniform mat4 projection;
out vec3 pos;
out vec3 color;
void main() {
    gl_Position = projection * view * aModel * vec4(aPos, 1.0);
    pos = aPos;
    color = aColor;
}