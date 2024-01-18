// This currently matches with six_vertex.wgsl
// Based on https://raytracing.github.io/books/RayTracingInOneWeekend.html
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

struct Uniforms {
    time: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct Light {
    color: vec3<f32>,
    direction: vec3<f32>,
};

struct Material {
    color: vec3<f32>,
    diffuse: f32,
    specular: f32,
};

struct Intersect { // hit
    len: f32,
    normal: vec3<f32>,
    material: Material,
};

struct Sphere {
    radius: f32,
    position: vec3<f32>,
    material: Material,
};

struct Plane {
    normal: vec3<f32>,
    material: Material,
};

const EPSILON: f32 = 1e-3;
const SAMPLES: i32 = 16;

const EXPOSURE: f32 = 1e-2;
const GAMMA: f32 = 2.2;
const INTENSITY: f32 = 100.0;

fn intersect_sphere(ray: Ray, sphere: Sphere) -> Intersect {
    let miss: Intersect = Intersect(0.0, vec3(0.0), Material(vec3(0.0), 0.0, 0.0));

    let oc = sphere.position - ray.origin;
    let l = dot(ray.direction, oc);
    let det = pow(l, 2.0) - dot(oc, oc) + pow(sphere.radius, 2.0);
    if (det < 0.0) {
        return miss;
    }

    var len = l - sqrt(det);
    if (len < 0.0) {
        len = l + sqrt(det);
    }
    if (len < 0.0) {
        return miss;
    }

    return Intersect(len, (ray.origin + len * ray.direction - sphere.position) / sphere.radius, sphere.material);
}

fn intersect_plane(ray: Ray, plane: Plane) -> Intersect {
    let miss: Intersect = Intersect(0.0, vec3(0.0), Material(vec3(0.0), 0.0, 0.0));

    let len = -dot(ray.origin, plane.normal) / dot(ray.direction, plane.normal);
    if (len < 0.0) {
        return miss;
    }

    return Intersect(len, plane.normal, plane.material);
}

fn trace(ray: Ray) -> Intersect {
    let miss: Intersect = Intersect(0.0, vec3(0.0), Material(vec3(0.0), 0.0, 0.0));

    let s1 = Sphere(2.0, vec3(-4.0, 3.0, 0.0), Material(vec3(1.0, 0.0, 0.2), 1.0, 0.001));
    let s2 = Sphere(3.0, vec3(4.0 ,3.0, 0.0), Material(vec3(0.0, 0.2, 1.0), 1.0, 0.0));
    let s3 = Sphere(1.0, vec3(0.5, 1.0, 6.0),  Material(vec3(1.0, 1.0, 1.0), 0.5, 0.25));
    let s4 = Sphere(1.0, vec3(6.0, 1.0, 4.0), Material(vec3(0.0, 1.0, 0.2), 0.5, 0.1));
    var p1 = Plane(vec3(0.0, 1.0, 0.0), Material(vec3(1.0, 1.0, 1.0), 1.0, 0.0));

    var intersection = miss;
    var plane = intersect_plane(ray, p1);
    if (plane.material.diffuse > 0.0 || plane.material.specular > 0.0) {
        intersection = plane;
    }
    var sphere = intersect_sphere(ray, s1);
    if (sphere.material.diffuse > 0.0 || sphere.material.specular > 0.0) {
        intersection = sphere;
    }
    sphere = intersect_sphere(ray, s2);
    if (sphere.material.diffuse > 0.0 || sphere.material.specular > 0.0) {
        intersection = sphere;
    }
    sphere = intersect_sphere(ray, s4);
    if (sphere.material.diffuse > 0.0 || sphere.material.specular > 0.0) {
        intersection = sphere;
    }
    sphere = intersect_sphere(ray, s3);
    if (sphere.material.diffuse > 0.0 || sphere.material.specular > 0.0) {
        intersection = sphere;
    }
    
    return intersection;
}

fn is_vec_zero(v: vec3<f32>) -> bool {
    if (v.x == 0.0 && v.y == 0.0 && v.z == 0.0) {
        return true;
    }
    return false;
}

fn is_miss(intersect: Intersect) -> bool {
    let a = intersect.len == 0.0;
    let b = is_vec_zero(intersect.normal);
    let c = is_vec_zero(intersect.material.color);
    let d = intersect.material.diffuse == 0.0;
    let e = intersect.material.specular == 0.0;
    if (a && b && c && d && e) {
        return true;
    }
    return false;
}

fn radiance(r: Ray) -> vec3<f32> {
    let ambient: vec3<f32> = vec3(0.6, 0.8, 1.0) * INTENSITY / GAMMA;
    var ray = r;
    let miss: Intersect = Intersect(0.0, vec3(0.0), Material(vec3(0.0), 0.0, 0.0));
    let light = Light(vec3(1.0) * INTENSITY, normalize(vec3(cos(uniforms.time / 10f), 0.75, sin(uniforms.time / 10f))));
    var color = vec3(0.0);
    var fresnel = vec3(0.0);
    var mask = vec3(1.0);
    for (var i: i32 = 0; i <= 16; i += 1) {
        let hit = trace(ray);
        if (hit.material.diffuse > 0.0 || hit.material.specular > 0.0) {
            let r0 = hit.material.color * hit.material.specular;
            let hv = clamp(dot(hit.normal, -ray.direction), 0.0, 1.0);
            fresnel = r0 + (1.0 - r0) * pow(1.0 - hv, 5.0);
            mask *= fresnel;

            let result = trace(Ray(ray.origin + hit.len * ray.direction + EPSILON * light.direction, light.direction));
            if (is_miss(result)) {
                color += clamp(dot(hit.normal, light.direction), 0.0, 1.0) * light.color
                       * hit.material.color.rgb * hit.material.diffuse
                       * (1.0 - fresnel) * mask / fresnel;
            }

            let reflection = reflect(ray.direction, hit.normal);
            ray = Ray(ray.origin + hit.len * ray.direction + EPSILON * reflection, reflection);
        } else {
            let spotlight = vec3(1e6) * pow(abs(dot(ray.direction, light.direction)), 250.0);
            color += mask * (ambient + spotlight);
            break;
        }
    }

    return color;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let width = 1600.0;
    let height = 1200.0;
    let resolution: vec2<f32> = vec2(width, height);
    let aspect_ratio = width / height;
    var uv = 2.0 * in.position.xy / resolution.xy - vec2(1.0); // Maps xy to [-1, 1]
    uv.x *= aspect_ratio;
    uv.y = -uv.y;

    let ray = Ray(vec3(0.0, 2.5, 12.0), normalize(vec3(uv.x, uv.y, -1.0)));
    let color = vec4(pow(radiance(ray) * EXPOSURE, vec3(1.0 / GAMMA)), 1.0);
    return color;
}
