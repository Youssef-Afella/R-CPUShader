use crate::common::*;

const MAX_BOUNCES: u32 = 4;
const SPHERES_COUNT: usize = 7;

struct HitData{
    emission: Vec3,
    color: Vec3,
    normal: Vec3,
    rougthness: f32,
    distance: f32,
}

const SPHERES: [Vec4; SPHERES_COUNT] = [
    Vec4::new(1002.0, 0.0, 0.0, 1000.0),
    Vec4::new(-1002.0, 0.0, 0.0, 1000.0),
    Vec4::new(0.0, 1002.0, 0.0, 1000.0),
    Vec4::new(0.0, -1002.0, 0.0, 1000.0),
    Vec4::new(0.0, 0.0, 1002.0, 1000.0),
    Vec4::new(0.0, -1.0, 0.0, 0.5),
    Vec4::new(0.0, 0.5, 0.0, 0.25),
];

fn ray_sphere(ro: Vec3, rd: Vec3, so: Vec3, sr: f32) -> f32
{
    let v = ro - so;
    let b = 2.0 * rd.dot(v);
    let c = v.dot(v) - (sr * sr);
    if b * b - 4.0 * c < 0.0
    {
        return -1.0;
    }
    return (-b - ((b * b) - 4.0 * c).sqrt()) * 0.5;
}

fn next_rand(state: &mut u32) -> u32
{
    *state = state.wrapping_mul(747796405).wrapping_add(2891336453);
    let shift = (*state >> 28) as u32;
    let mut result = ((*state >> (shift + 4)) ^ *state).wrapping_mul(277803737);
    result = (result >> 22) ^ result;
    result
}

fn rand_1(state: &mut u32) -> f32
{
    return next_rand(state) as f32 / 4294967295.0;
}

fn rand_1_nd(state: &mut u32) -> f32
{
    let theta = 2.0 * 3.1415926 * rand_1(state);
    let rho = (-2.0 * rand_1(state).ln()).sqrt();
    return rho * theta.cos();
}

fn rand_dir(state: &mut u32) -> Vec3
{
    let x = rand_1_nd(state);
    let y = rand_1_nd(state);
    let z = rand_1_nd(state);
    return vec3(x, y, z).normalize();
}

fn intersect(ro: Vec3, rd: Vec3) -> HitData
{
    let mut hit = HitData{
        emission: Vec3::ZERO,
        color: Vec3::ZERO,
        normal: Vec3::ONE,
        rougthness: 1.0,
        distance: 10000.0
    };

    for i in 0..SPHERES_COUNT
    {
        let pos = vec3(SPHERES[i].x, SPHERES[i].y, SPHERES[i].z);
        let rad = SPHERES[i].w;

        let l = ray_sphere(ro, rd, pos, rad);
                
        if l < hit.distance && l > 0.001
        {
            let p = ro + rd * l;
            let normal = (p - pos).normalize();
            hit.normal = normal;

            if i == 0 {
                hit.color = vec3(1.0, 0.0, 0.0);
            }
            else if i == 1 {
                hit.color = vec3(0.0, 1.0, 0.0);
            }
            else{
                hit.color = Vec3::ONE;
            }

            if i == 5 {
                hit.rougthness = 0.0;
            }
            else{
                hit.rougthness = 1.0;
            }

            if i == SPHERES_COUNT-1 {
                hit.emission = Vec3::ONE * 20.0;
            }
            else{
                hit.emission = Vec3::ZERO;
            }

            hit.distance = l;
        }
    }

    return hit;
}

fn trace(mut ro: Vec3, mut rd: Vec3, state: &mut u32) -> Vec3
{
    let mut ray_light = Vec3::ZERO;
    let mut ray_color = Vec3::ONE;

    for _i in 0..MAX_BOUNCES
    {
        let hit: HitData = intersect(ro, rd);

        if hit.distance > 9999.0 {
            break;
        }

        ray_light = ray_light + hit.emission * ray_color;
        ray_color = ray_color * hit.color;

        ro = ro + rd * hit.distance;

        let diffuse_dir = (hit.normal + rand_dir(state)).normalize();
        let specular_dir = rd.reflect(hit.normal);
        rd = specular_dir.lerp(diffuse_dir, hit.rougthness).normalize();
    }

    return ray_light;
}

pub fn main(frag_coord: Vec2, resolution: Vec2, _time: f32, frame: u32) -> Vec4
{
    let uv = (frag_coord - resolution * 0.5) / resolution.y;

    let ray_origin = vec3(0.0, 0.0, -3.0);
    let ray_point = vec3(uv.x * 2.0, uv.y * 2.0, -1.75);
    let ray_dir = (ray_point - ray_origin).normalize();

    let mut state = (frag_coord.x + frag_coord.y * resolution.x) as u32 + frame * 78423;
    let t = trace(ray_origin, ray_dir, &mut state);

    return vec4(t.x, t.y, t.z, 1.0);
}