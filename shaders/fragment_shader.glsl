#version 460 core
out vec4 FragColor;
in vec3 pos;
in vec3 vColor;
in vec2 texCoord;
in vec3 Normal;
in vec3 FragPos;


uniform float time;
uniform sampler2D tex;

struct DirLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir, vec3 tex) {
    // ambient
    vec3 ambient = light.ambient * tex;

    // diffuse
    vec3 norm = normalize(normal);
    vec3 lightDir = normalize(-light.direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * tex;

    return ambient + diffuse;
}

uniform DirLight dirLight;
uniform vec3 viewPos;

void main() {
    vec3 text = texture(tex, texCoord).rgb;
    vec3 debugColor = Normal * 0.5 + 0.5;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 result = text * CalcDirLight(dirLight, Normal, viewDir, text);


    FragColor = vec4(result, 1.0);
}
