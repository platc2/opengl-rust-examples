#version 330 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 normal;

out vec4 frag_color;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;

uniform uint shininess;

void main() {
    float ambientStrenght = 0.1;
    vec3 ambient = ambientStrenght * lightColor;

    vec3 norm = normalize(normal);
    vec3 lightDir = normalize(lightPos - pos);
    float diff = max(dot(norm, lightDir), 0.);
    vec3 diffuse = diff * lightColor;

    vec3 viewDir = normalize(viewPos - pos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.), shininess);
    vec3 specular = .5 * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;
    frag_color = vec4(result, 1.);
}
