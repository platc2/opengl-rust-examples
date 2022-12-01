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
    uint num_inscatter_points;
    uint num_optical_depth_points;
    float g;
    float intensity;
    float rayleigh_scale_height;
    float mie_scale_height;
};

layout (location = 0) uniform sampler2D tex;

layout (location = 0) out vec4 color;


struct Sphere {
    vec3 centre;
    float radius;
};

struct Ray {
    vec3 origin;
    vec3 direction;
};

struct HitResult {
    float enter_distance;
    float exit_distance;
};


const vec3 PLANET_CENTRE = vec3(0, 0, 0);
const float PI = 3.141592;

const vec3 rayleigh_coefficients = vec3(
    3.8e-6,
    13.5e-6,
    33.1e-6
);

const vec3 mie_coefficients = vec3(2e-7);

/**
 * Computes the intersection points of a ray and a sphere. Returns a tuple as hit result where both value
 * indicate the distance when the ray enters and exits the sphere.
 */
bool ray_sphere(const Sphere sphere, const Ray ray, out HitResult hit_result);

/**
 * Calculates the light with rayleigh scattering.
 */
vec3 calculate_light(const Ray ray, const float ray_length, const vec3 sun_direction, const vec3 original_color);


void main() {
    vec3 sun_direction = normalize(vec3(0, cos(time * PI * 2), sin(time * PI * 2)));
    const Ray camera = Ray(camera_position, normalize(vec3(uv, 0) - vec3(0.5, 0.5, 1)));

    const vec4 original_color = texture(tex, uv);

    HitResult planet_hit_result;
    const bool planet_hit = ray_sphere(Sphere(PLANET_CENTRE, planet_radius), camera, planet_hit_result);
    const bool inside_planet = planet_hit && planet_hit_result.enter_distance < 0.0 && planet_hit_result.exit_distance > 0.0;

    color.xyz = original_color.xyz;
    if (!inside_planet) {
        HitResult atmosphere_hit_result;
        const bool atmosphere_hit = ray_sphere(Sphere(PLANET_CENTRE, atmosphere_radius + planet_radius), camera, atmosphere_hit_result);

        const bool render_skydome = atmosphere_hit && atmosphere_hit_result.exit_distance > 0.0;
        if (render_skydome) {
            const float epsilon = 1e2;
            // Compute distance to atmosphere - If we are inside of it, the distance equals 0
            const float distance_to_atmosphere = max(0.0, atmosphere_hit_result.enter_distance);
            const float planet_distance = original_color.w;
            const float distance_through_atmosphere =
            (planet_hit && planet_hit_result.enter_distance >= 0.0 ? min(planet_hit_result.enter_distance, atmosphere_hit_result.exit_distance)
            : atmosphere_hit_result.exit_distance) - distance_to_atmosphere;

            const vec3 point_in_atmosphere = (camera.direction * distance_to_atmosphere + epsilon) + camera.origin;
            color.xyz = calculate_light(Ray(point_in_atmosphere, camera.direction), distance_through_atmosphere - epsilon * 2.0,
                                        sun_direction, original_color.xyz);
        }
    }

    color.rgb = 1.0 - exp(-color.rgb);
/*
    color.r = color.r < 1.413 ? pow(color.r * 0.38317, 1.0 / 2.2) : 1.0 - exp(-color.r);
    color.g = color.g < 1.413 ? pow(color.g * 0.38317, 1.0 / 2.2) : 1.0 - exp(-color.g);
    color.b = color.b < 1.413 ? pow(color.b * 0.38317, 1.0 / 2.2) : 1.0 - exp(-color.b);
*/
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
// Scattering
//////////////////////////////////////////////////////////////////////////////////////////

/** Computes the height above the surface of a sphere */
float surface_height(const Sphere sphere, const vec3 sample_point) {
    const float distance_from_centre = length(sample_point - sphere.centre);
    const float height_above_surface = distance_from_centre - sphere.radius;
    return height_above_surface;
}

/** Computes the atmospheric density for a point */
float atmospheric_density(const Sphere sphere, const vec3 sample_point, const float scale_height) {
    return exp(-surface_height(sphere, sample_point) / scale_height);
}

vec2 optical_depth(const Sphere sphere, const Ray ray, const float ray_length, const float atmosphere_height) {
    const float step_size = ray_length / (num_optical_depth_points - 1);
    vec2 accumulated_density = vec2(0.0);

    for (int i = 0; i < num_optical_depth_points; ++i) {
        const vec3 sample_point = ray.origin + (ray.direction * step_size * i);
        const float local_rayleigh_density = atmospheric_density(sphere, sample_point, rayleigh_scale_height);
        const float local_mie_density = atmospheric_density(sphere, sample_point, mie_scale_height);
        accumulated_density += vec2(local_rayleigh_density, local_mie_density);
    }

    return step_size * accumulated_density;
}

float phase_rayleigh(const float cos_theta) {
    return (3.0 / 16.0 * PI) * (1.0 + cos_theta * cos_theta);
}

float phase(const float cos_theta, const float g) {
    if (g == 0.0) {
        return phase_rayleigh(cos_theta);
    } else {
        const float a = 1.5 * (1.0 - g * g) / (2.0 + g * g);
        const float b = (1.0 + cos_theta * cos_theta) / pow(1.0 + g * g - 2.0 * g * cos_theta, 1.5);
        return a * b;
    }
}

/**
 * Calculates the light with rayleigh scattering.
 */
vec3 calculate_light(const Ray ray, const float ray_length, const vec3 sun_direction, const vec3 original_color) {
    const float step_size = ray_length / (num_inscatter_points - 1);
    vec2 total_view_ray_optical_depth = vec2(0);

    const float mu = dot(ray.direction, sun_direction);
    const float phaseR = phase(mu, 0.0);
    const float phaseM = phase(mu, g);

    const Sphere planet = Sphere(PLANET_CENTRE, planet_radius);
    vec3 scattered_light = vec3(0.0);
    for (int i = 0; i < num_inscatter_points; ++i) {
        const vec3 in_scatter_point = ray.origin + step_size * i * ray.direction;
        HitResult sun_ray_hit_result;
        // This must intersect as we are within the atmosphere
        ray_sphere(Sphere(PLANET_CENTRE, atmosphere_radius + planet_radius), Ray(in_scatter_point, sun_direction), sun_ray_hit_result);

        const float sun_ray_length = sun_ray_hit_result.exit_distance;
        const vec2 sun_ray_optical_depth = optical_depth(planet, Ray(in_scatter_point, sun_direction), sun_ray_length, atmosphere_radius);
        const vec2 current_optical_depth = optical_depth(planet, ray, step_size * i, atmosphere_radius);
        total_view_ray_optical_depth += current_optical_depth;
        const vec3 attenuation = exp(-rayleigh_coefficients * (current_optical_depth.x + sun_ray_optical_depth.x) - mie_coefficients * 1.1 * (current_optical_depth.y + sun_ray_optical_depth.y));
        scattered_light += phaseR * attenuation * rayleigh_coefficients;
        scattered_light += phaseM * attenuation * mie_coefficients;
    }

//    const vec2 total_view_depth = optical_depth(planet, ray, ray_length, atmosphere_radius);
    const vec2 total_view_depth = total_view_ray_optical_depth;
    const vec3 original_color_attenuated = original_color * exp(-rayleigh_coefficients * total_view_depth.x - mie_coefficients * 1.1 * total_view_depth.y);
    return original_color_attenuated + scattered_light * intensity * step_size;
}
