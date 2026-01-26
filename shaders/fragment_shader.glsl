#version 460 core
out vec4 FragColor;
in vec3 pos;
in vec3 color;
in vec2 texCoord;

uniform float time;
uniform sampler2D tex;
void main() {
    vec3 color = 0.5 + 0.5 * cos(time + vec3(0.0, 2.0, 4.0));
    if (texCoord != vec2(0, 0)) {
        FragColor = texture(tex, texCoord);
    } else {
        FragColor = vec4(color, 1.0);
    }
}
