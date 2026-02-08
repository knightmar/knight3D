#version 330 core
out vec4 FragColor;
in vec3 pos;
in vec3 vColor;
in vec2 texCoord;
in vec3 Normal;
in vec3 FragPos;

struct DirLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

struct Material {
    sampler2D ambient;
    sampler2D specular;
    float shininess;
    bool hasSpecularMap;
};

uniform int lightsCount;
uniform float time;
uniform Material material;
uniform DirLight dirLight;
uniform vec3 viewPos;
uniform PointLight[256] pointLights;

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir, vec3 diffuse_text, vec3 specular_text) {
    // ambient
    vec3 ambient = light.ambient * diffuse_text;

    // diffuse
    vec3 norm = normalize(normal);
    vec3 lightDir = normalize(-light.direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * diffuse_text;

    // specular
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * specular_text;

    return ambient + diffuse + specular;
}

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 viewDir, vec3 diffuse_text, vec3 specular_text) {
    float distance = length(light.position - FragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance +
    light.quadratic * (distance * distance));
    vec3 lightDir = normalize(light.position - FragPos);


    // ambient
    vec3 ambient = light.ambient * diffuse_text * attenuation;

    // diffuse
    vec3 norm = normalize(normal);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * diffuse_text * attenuation;

    // specular
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * specular_text * attenuation;

    return ambient + diffuse + specular;
}

void main() {
    vec3 diffuse_text = texture(material.ambient, texCoord).rgb;
    vec3 specular_text;

    if (material.hasSpecularMap) {
        specular_text = texture(material.specular, texCoord).rgb;
    } else {
        specular_text = vec3(0.0);
    }

    vec3 debugColor = Normal * 0.5 + 0.5;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 result = CalcDirLight(dirLight, Normal, viewDir, diffuse_text, specular_text);
    for (int i = 0; i < lightsCount; i++) {
        result += CalcPointLight(pointLights[i], Normal, viewDir, diffuse_text, specular_text);
    }

    FragColor = vec4(result, 1.0);
}
