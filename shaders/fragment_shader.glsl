#version 460 core
out vec4 FragColor;
in vec3 pos;
in vec3 vColor;
in vec2 texCoord;

uniform vec4 uTint;
uniform bool uUseTexture;
uniform float time;
uniform sampler2D tex;
void main() {
    vec3 col = 0.5 + 0.5 * cos(time + vec3(0.0, 2.0, 4.0));
    FragColor = vec4(col, 1.0);

    if (texCoord != vec2(0, 0)) {
        FragColor = texture(tex, texCoord);
    } else {
        if (vColor[0] > 0) {
            FragColor = vec4(vColor, 1.0);
        }
        else {
            FragColor = vec4(col, 1.0);
        }
    }
}
