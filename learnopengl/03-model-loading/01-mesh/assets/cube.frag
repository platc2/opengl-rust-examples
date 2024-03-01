#version 330 core


struct Material {
    sampler2D texture_diffuse1;
    sampler2D texture_specular1;
    float shininess;
};


struct DirLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};


struct PointLight {
    vec3 position;

    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};


struct SpotLight {
    vec3 position;
    vec3 direction;

    float constant;
    float linear;
    float quadratic;

    float cutoff;
    float outerCutoff;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};


vec3 CalcDirLight(DirLight dirLight, vec3 normal, vec3 viewDir);
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir);


layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coords;

out vec4 frag_color;

uniform vec3 viewPos;

uniform Material material;
uniform DirLight dirLight;
#define NUM_POINT_LIGHTS (4)
uniform PointLight pointLights[NUM_POINT_LIGHTS];
uniform SpotLight spotLight;

void main() {
    vec3 norm = normalize(normal);
    vec3 viewDir = normalize(viewPos - pos);

    vec3 result = CalcDirLight(dirLight, norm, viewDir);
    for (int i = 0; i < NUM_POINT_LIGHTS; ++i) {
        result += CalcPointLight(pointLights[i], norm, pos, viewDir);
    }
    result += CalcSpotLight(spotLight, norm, pos, viewDir);

    frag_color = vec4(result, 1.);

    frag_color = texture(material.texture_diffuse1, tex_coords);
}


vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir) {
    vec3 lightDir = normalize(-light.direction);

    float diff = max(dot(normal, lightDir), 0.);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.), material.shininess);

    vec3 ambient = light.ambient * vec3(texture(material.texture_diffuse1, tex_coords));
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.texture_diffuse1, tex_coords));
    vec3 specular = light.specular * spec * vec3(texture(material.texture_specular1, tex_coords));

    return ambient + diffuse + specular;
}


vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
    vec3 lightDir = normalize(light.position - pos);

    float diff = max(dot(normal, lightDir), 0.);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.), material.shininess);

    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    vec3 ambient  = light.ambient  * vec3(texture(material.texture_diffuse1, tex_coords));
    vec3 diffuse  = light.diffuse  * diff * vec3(texture(material.texture_diffuse1, tex_coords));
    vec3 specular = light.specular * spec * vec3(texture(material.texture_specular1, tex_coords));

    return attenuation * (ambient + diffuse + specular);
}


vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir) {
    vec3 lightDir = normalize(light.position - pos);

    float diff = max(dot(normal, lightDir), 0.);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    vec3 ambient  = light.ambient  * vec3(texture(material.texture_diffuse1, tex_coords));
    vec3 diffuse  = light.diffuse  * diff * vec3(texture(material.texture_diffuse1, tex_coords));
    vec3 specular = light.specular * spec * vec3(texture(material.texture_specular1, tex_coords));

    float theta = dot(lightDir, normalize(-light.direction));
    float epsilon = light.cutoff - light.outerCutoff;
    float intensity = clamp((theta - light.outerCutoff) / epsilon, 0., 1.);
    return ambient + (diffuse + specular) * intensity * attenuation;
}
