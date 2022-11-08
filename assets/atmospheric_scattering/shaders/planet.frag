#version 430 core
#extension GL_ARB_enhanced_layouts: enable


layout (location = 0) in SHADER_VARYING {
    vec2 uv;
};

layout (std140, binding = 0) uniform CameraSettings {
    vec3 camera_position;
};

layout (std140, binding = 1) uniform WorldSettings {
    float time;
    float planet_radius;
    float atmosphere_radius;
};

layout (location = 0) out vec4 color;


const vec3 PLANET_CENTRE = vec3(0, 0, 0);
const float PI = 3.141592;

struct Ray {
    vec3 origin;
    vec3 direction;
};

struct Sphere {
    vec3 centre;
    float radius;
};

struct HitResult {
    float enter_distance;
    float exit_distance;
};

/**
 * Computes the intersection points of a ray and a sphere. Returns a tuple as hit result where both value
 * indicate the distance when the ray enters and exits the sphere.
 */
bool ray_sphere(const Sphere sphere, const Ray ray, out HitResult hit_result);

/**
 * Compute the scene color given
 * - stars
 * - planet
 * - sun
 * The depth is contained in the 4th component 'w'
 */
vec4 scene_color(const Ray ray, const vec3 sun_direction);

/**
 * Generic 3D noies function
 */
float noise(const vec3 p);

void main() {
    const vec3 sun_direction = normalize(vec3(0, cos(time * PI * 2), sin(time * PI * 2)));
    const Ray camera = Ray(camera_position, normalize(vec3(uv, 0) - vec3(0.5, 0.5, 1)));
    color = scene_color(camera, sun_direction);
}

//////////////////////////////////////////////////////////////////////////////////////////
// Collision Detection
//////////////////////////////////////////////////////////////////////////////////////////

/**
 * Computes the intersection points of a ray and a sphere. Returns a tuple as hit result where both value
 * indicate the distance when the ray enters and exits the sphere.
 */
bool ray_sphere(const Sphere sphere, const Ray ray, out HitResult hit_result) {
    const vec3 offset = ray.origin - sphere.centre;
    const float a = 1.0;  // Ray direction should be normalized
    const float b = 2.0 * dot(ray.direction, offset);
    const float c = dot(offset, offset) - (sphere.radius * sphere.radius);

    const float delta = b * b - 4.0 * a * c;
    if (delta < 0.0) {
        // No intersection
        return false;
    }

    const float two_a = 2.0 * a;
    if (delta == 0.0) {
        // One intersection point, i.e. ray is tangent to sphere
        const float distance = -(b / two_a);
        hit_result.enter_distance = distance;
        hit_result.exit_distance = distance;
    } else {
        // Two intersection points
        const float delta_sqrt = sqrt(delta);
        hit_result.enter_distance = -(b + delta_sqrt) / two_a;
        hit_result.exit_distance = -(b - delta_sqrt) / two_a;
    }

    return true;
}

//////////////////////////////////////////////////////////////////////////////////////////
// Planet rendering
//////////////////////////////////////////////////////////////////////////////////////////

void star_colors(inout vec4 color, const vec3 ray_direction) {
    // Totally random numbers generates us a starfield
    const float star_coefficient = ray_direction.x * 102502 + ray_direction.y * 5201000;
    if (int(floor(star_coefficient)) % 349 == 0) {
        const float intensity_coefficient = abs(sin(abs(ray_direction.x) * 5201000 + abs(ray_direction.y)));
        color.xyz = vec3(intensity_coefficient);
    }
}

void sun_color(inout vec4 color, const vec3 ray_direction, const vec3 sun_direction) {
    const float sun_dot = dot(ray_direction, sun_direction);
    const float SUN_COEFF = 0.9998;
    if (sun_dot > SUN_COEFF) {
        const float coeff01 = (sun_dot - SUN_COEFF) / (1.0 - SUN_COEFF);
        const float gradient = min(
            exp(2.0 * coeff01 - 1) - 1,
            exp(coeff01 / 3) - 0.5);
        if (gradient > 0.1) {
            color.xyz = vec3(gradient);
        }
    }
}

void planet_color(inout vec4 color, const Ray ray, const Sphere sphere, const vec3 sun_direction) {
    HitResult planet_hit;
    if (ray_sphere(sphere, ray, planet_hit) && planet_hit.enter_distance >= 0.0) {
        color.xyz = vec3(0.0, 0.25, 0.05);
        color.w = planet_hit.enter_distance;

        const vec3 planet_position = ray.origin + ray.direction * planet_hit.enter_distance - sphere.centre;
        const vec3 surface_normal = normalize(planet_position);

        float noise_value = 0;
        for (int i = 0; i < 8; ++i) {
            // Generate layered noise
            noise_value += noise(planet_position * (2 * (i + 1)) / planet_radius) * pow(2.0, -(float(i) + 1.0));
        }

        if (noise_value < 0.3) {
            color.xyz = vec3(0, 0, 0.75);
        } else if (noise_value < 0.45) {
            color.xyz = vec3(0, 0, 1);
        } else if (noise_value < 0.5) {
            color.xyz = vec3(0.25, 0.5, 1);
        } else if (noise_value < 0.52) {
            color.xyz = vec3(1, 1, 0);
        } else if (noise_value < 0.6) {
            color.xyz = vec3(0, 0.5, 0.2);
        } else if (noise_value < 0.65) {
        } else if (noise_value < 0.7) {
            color.xyz = vec3(0.2);
        } else if (noise_value < 0.725) {
            color.xyz = vec3(0.5);
        } else {
            color.xyz = vec3(1);
        }

        const float diffuse = max(0.025, dot(surface_normal, sun_direction));
        color.xyz *= vec3(diffuse);
    }
}

/**
 * Compute the scene color given
 * - stars
 * - planet
 * - sun
 * The depth is contained in the 4th component 'w'
 */
vec4 scene_color(const Ray ray, const vec3 sun_direction) {
    vec4 color = vec4(vec3(0), 1e16);
    star_colors(color, ray.direction);
    sun_color(color, ray.direction, sun_direction);
    planet_color(color, ray, Sphere(PLANET_CENTRE, planet_radius), sun_direction);
    return color;
}

//////////////////////////////////////////////////////////////////////////////////////////
// Noise generation
//////////////////////////////////////////////////////////////////////////////////////////

float mod289(const float x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec4 mod289(const vec4 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec4 perm(const vec4 x) {
    return mod289(((x * 34.0) + 1.0) * x);
}

/**
 * Generic 3D noies function
 */
float noise(const vec3 p) {
    const vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    const vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    const vec4 k1 = perm(b.xyxy);
    const vec4 k2 = perm(k1.xyxy + b.zzww);

    const vec4 c = k2 + a.zzzz;
    const vec4 k3 = perm(c);
    const vec4 k4 = perm(c + 1.0);

    const vec4 o1 = fract(k3 * (1.0 / 41.0));
    const vec4 o2 = fract(k4 * (1.0 / 41.0));

    const vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    const vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);

    return o4.y * d.y + o4.x * (1.0 - d.y);
}
