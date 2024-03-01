#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    sampler2D emission;
    float shininess;
};

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
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
    vec3 lightDir = normalize(light.position - pos);
    float diff = max(dot(norm, lightDir), 0.);

    vec3 viewDir = normalize(viewPos - pos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.), material.shininess);

    vec3 ambient = light.ambient * diffuse_value;
    vec3 diffuse = light.diffuse * (diff * diffuse_value);
    vec3 specular = light.specular * (spec * specular_value);
    vec3 emission = abs(sin(time)) * vec3(texture(material.emission, tex_coords));

    vec3 result = ambient + diffuse + specular + emission;
    frag_color = vec4(result, 1.);
}
