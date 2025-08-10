#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

uniform float time;
out vec3 pos;
out vec3 color;
out vec2 texCoord;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    pos = aPos;
    color = aColor;
    texCoord = aTexCoord;
}