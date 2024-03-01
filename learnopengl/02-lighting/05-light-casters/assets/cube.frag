#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 direction;
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;

    float cutoff;
    float outerCutoff;

    uint type;
};

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coords;

out vec4 frag_color;

uniform vec3 viewPos;

uniform Material material;
uniform Light light;
uniform float time;

void main() {
    vec3 diffuse_value = vec3(texture(material.diffuse, tex_coords));
    vec3 specular_value = vec3(texture(material.specular, tex_coords));

    vec3 norm = normalize(normal);
    vec3 lightDir;

    switch (light.type) {
        case 0:
            lightDir = normalize(-light.direction);
            break;
        case 1:
        case 2:
            lightDir = normalize(light.position - pos);
            break;
    }
    float diff = max(dot(norm, lightDir), 0.);

    vec3 viewDir = normalize(viewPos - pos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.), material.shininess);

    vec3 ambient = light.ambient * diffuse_value;
    vec3 diffuse = light.diffuse * (diff * diffuse_value);
    vec3 specular = light.specular * (spec * specular_value);

    vec3 result = ambient;
    if (light.type == 2u) {
        float theta = dot(lightDir, normalize(-light.direction));
        float epsilon = light.cutoff - light.outerCutoff;
        float intensity = clamp((theta - light.outerCutoff) / epsilon, 0., 1.);
        result += (diffuse + specular) * intensity;
    } else {
        result += diffuse + specular;
    }

    if (light.type == 1u) {
        float distance = length(light.position - pos);
        float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
        result *= attenuation;
    }

    frag_color = vec4(result, 1.);
}
