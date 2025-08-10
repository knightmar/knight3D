#version 460 core
out vec4 FragColor;
in vec3 pos;
in vec3 color;
in vec2 texCoord;

uniform float time;
uniform sampler2D tex;
void main() {
    FragColor = texture(tex, texCoord);
}