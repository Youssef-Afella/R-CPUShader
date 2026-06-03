use crate::common::*;

const MAX_BOUNCES: u32 = 4;
const NEAR_DISK_COUNT: u32 = 3;
const FAR_DISK_COUNT: u32 = 6;
const DIRECTIONAL_SAMPLES: u32 = 32;

struct HitData{
    surface_type: u32,
    color: Vec3,
    normal: Vec2,
    distance: f32,
}

fn ray_circle(ro: Vec2, rd: Vec2, c: Vec2, r: f32) -> f32
{
    let oc = ro - c;

    let b = oc.dot(rd);
    let c0 = oc.dot(oc) - r * r;
    let mut h = b * b - c0;

    if h < 0.0 {
        return -1.0;
    }

    h = h.sqrt();

    let mut t = -b - h;
    if t < 0.0 { t = -b + h; }
    if t < 0.0 { return -1.0;}

    return t;
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

fn intersect(ro: Vec2, rd: Vec2) -> HitData
{
    let mut hit = HitData { 
        surface_type: 0,
        color: Vec3::ZERO,
        normal: Vec2::ONE, 
        distance: 10000.0
    };
    
    let light = ray_circle(ro, rd, vec2(0.1, 0.15), 0.03);
    if light > 0.0
    {
        hit.distance = light;
        hit.color = Vec3::ONE*6.0;
        hit.surface_type = 0;
    }
    
    for i in 0..NEAR_DISK_COUNT
    {
        let angle = 2.0 * PI * i as f32 / NEAR_DISK_COUNT as f32;
        let pos = vec2(angle.cos(), angle.sin()) * 0.125;
        let radius = 0.075;
        
        let t = ray_circle(ro, rd, pos, radius);
        
        if t < hit.distance && t > 0.0
        {
            hit.distance = light;
            
            let is_entering = (ro - pos).length() > radius;
                
            let p = ro + rd * t;

            hit.normal = (p - pos).normalize() * if is_entering { 1.0 } else { -1.0 };
            hit.surface_type = if is_entering { 1 } else { 2 };
        }
    }
    
    for i in 0..FAR_DISK_COUNT
    {
        let angle = 2.0 * PI * (i as f32 + 0.25) / FAR_DISK_COUNT as f32;
        let pos = vec2(angle.cos(), angle.sin()) * 0.32;
        let radius = 0.1;
        
        let t = ray_circle(ro, rd, pos, radius);
        
        if t < hit.distance && t > 0.0
        {
            hit.distance = light;
            
            let is_entering = (ro - pos).length() > radius;
                
            let p = ro + rd * t;

            hit.normal = (p - pos).normalize() * if is_entering { 1.0 } else { -1.0 };
            hit.surface_type = if is_entering { 1 } else { 2 };
        }
    }
    
    return hit;
}

fn get_reflectance(i: Vec2, t: Vec2, nor: Vec2, iora: f32, iorb: f32) -> f32
{
    let cosi = i.dot(nor);
    let cost = t.dot(nor);
    let rs = (iora * cosi - iorb * cost) / (iora * cosi + iorb * cost);
    let rp = (iorb * cosi - iora * cost) / (iorb * cosi + iora * cost);
    return (rs * rs + rp * rp) * 0.5;
}


fn trace(mut ro: Vec2, mut rd: Vec2, ior: f32, state: &mut u32) -> Vec3
{
    const EPS:f32 = 0.0001;
    let mut transmittance = Vec3::ONE;

    for _i in 0..MAX_BOUNCES
    {
        let hit = intersect(ro, rd);

        if hit.distance > 9999.0 {
            break;
        }

        if hit.surface_type == 0
        {
            return hit.color * transmittance;
        }

        if hit.surface_type == 1 || hit.surface_type == 2
        {
            let is_entering = hit.surface_type == 1;//entering or exiting the glass
            ro = ro + rd * hit.distance;

            let reflected = rd.reflect(hit.normal);
            let refracted = rd.refract(hit.normal, if is_entering { 1.0/ior } else { ior });
            let reflectance  = get_reflectance(rd, refracted, hit.normal, 
                if is_entering { 1.0 } else { ior }, 
                if is_entering { ior } else { 1.0 });
            let reflect_it = rand_1(state) < reflectance;
            
            /*if(!is_entering){
                //Absorbtion
                trasmittance *= exp(-hit.distance * hit.color * ABSORBTION_STRENGTH);
            }*/

            if reflect_it
            {
                ro = ro + hit.normal * EPS;
                rd = reflected;
                transmittance = transmittance * reflectance;
            }
            else{
                ro = ro - hit.normal * EPS;
                rd = refracted;
                transmittance = transmittance * (1.0 - reflectance);
            }
        }
    }

    return Vec3::ZERO;
}

fn get_dispersed_color(w: f32) -> Vec3 
{
    let r = (w * PI * 2.0).sin();
    let g = ((w - 0.25) * PI * 2.0).sin();
    let b = ((w - 0.5) * PI * 2.0).sin();
    return vec3(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
}

pub fn main(frag_coord: Vec2, resolution: Vec2, _time: f32, frame: u32) -> Vec4
{
    let uv = (frag_coord - resolution * 0.5) / resolution.y;

    let mut state = (frag_coord.x + frag_coord.y * resolution.x) as u32 + frame * 78423;

    let angle = 2.0 * PI * (frame as f32 + rand_1(&mut state)) / DIRECTIONAL_SAMPLES as f32;
    let dir = vec2(angle.cos(), angle.sin());

    let spec = rand_1(&mut state);
    let ior = 1.05 + 0.1 * spec;
    let color_mask = get_dispersed_color(spec);

    let t = trace(uv, dir, ior, &mut state) * color_mask;

    return vec4(t.x, t.y, t.z, 1.0);
}