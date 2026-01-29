#version 460 core
out vec4 FragColor;
in vec3 pos;
in vec3 vColor;
in vec2 texCoord;

uniform float time;
uniform sampler2D tex;


struct Light {
    vec3 position;
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

#define NR_POINT_LIGHTS 4
uniform Light pointLights[NR_POINT_LIGHTS];

void main() {
    //    vec3 col = 0.5 + 0.5 * cos(time + vec3(0.0, 2.0, 4.0));
    FragColor = texture(tex, texCoord);

    //    if (texCoord != vec2(0, 0)) {
    //        FragColor = texture(tex, texCoord);
    //    } else {
    //        if (vColor[0] > 0) {
    //            FragColor = vec4(vColor, 1.0);
    //        }
    //        else {
    //            FragColor = vec4(col, 1.0);
    //        }
    //    }
}
