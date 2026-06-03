//Analytical solution to render a volumetric box lit with a single directional light without raymarching.
//Full Math/Implementation details are on this paper :
//https://github.com/Youssef-Afella/notes/blob/main/Analytical%20Volumetric%20Box.png

use crate::common::*;

//Volume Properties /////////////////////////////////////////
const ABSORPTION: f32 = 2.0;

#[inline]
fn beer(d: f32) -> f32
{
    return (-d * ABSORPTION).exp();
}

#[inline]
fn rayleigh(c: f32) -> f32 
{
    return 0.58904 * (1.0 + c * c);
}

#[inline]
fn phase(a: f32) -> f32
{
    return rayleigh(a);
}

//Geometry //////////////////////////////////////////////////
fn intersect_aabb(ro: Vec3, rd: Vec3, bmin: Vec3, bmax: Vec3) -> Vec2
{
    let inv = 1.0 / rd;
    
    let t0 = (bmin - ro) * inv;
    let t1 = (bmax - ro) * inv;

    let tmin = t0.min(t1);
    let tmax = t0.max(t1);

    let tnear = tmin.x.max(tmin.y).max(tmin.z);
    let tfar  = tmax.x.min(tmax.y).min(tmax.z);

    if tfar < 0.0 || tnear > tfar { return vec2(0.0, 0.0); }

    return vec2(tnear, tfar);
}

fn integrate_trapezoid(lt1: f32, lt2: f32, t1: f32, t2: f32) -> f32 {
    let a = (lt2 - lt1) / (t2 - t1);
    let b = lt1 - a * t1;

    let mut i = beer(t1 * (1.0 + a) + b) - beer(t2 * (1.0 + a) + b);
    i /= (1.0 + a) * ABSORPTION;

    return i.max(0.0);
}

fn intersect_plane_segment(p: Vec3, n: Vec3, a: Vec3, b: Vec3) -> Option<Vec3> {
    let da = n.dot(a - p);
    let db = n.dot(b - p);

    if da * db > 0.0 {
        return None;
    }

    let denom = da - db;
    if denom.abs() < 1e-5 {
        return None;
    }

    let t = da / denom;
    return Some(a.lerp(b, t));
}

fn coord_on_plane(p: Vec3, o: Vec3, u: Vec3, v: Vec3) -> Vec2 {
    // Return point coord on the plane in the basis (o, u, v)
    let d = p - o;

    let uu = u.dot(u);
    let uv = u.dot(v);
    let vv = v.dot(v);
    let du = d.dot(u);
    let dv = d.dot(v);

    let denom = uu * vv - uv * uv;

    let a = (du * vv - dv * uv) / denom;
    let b = (dv * uu - du * uv) / denom;

    return Vec2::new(a, b);
}

fn integrate_box(ro: Vec3, rd: Vec3, ld: Vec3, bmin: Vec3, bmax: Vec3) -> f32 {
    let mut integration = 0.0;
    let eps = 1e-5;

    let t = intersect_aabb(ro, rd, bmin, bmax);
    let d_dist = t.y - t.x;

    let p1 = ro + rd * (t.x + eps); // entry point
    let p2 = ro + rd * (t.y - eps); // exit point

    let l1 = intersect_aabb(p1, ld, bmin, bmax).y;
    let l2 = intersect_aabb(p2, ld, bmin, bmax).y;

    let q1 = p1 + ld * l1; // p1 projection
    let q2 = p2 + ld * l2; // p2 projection

    let diff = -(q1 - q2).abs();
    let step_x = if diff.x >= -eps { 1.0 } else { 0.0 };
    let step_y = if diff.y >= -eps { 1.0 } else { 0.0 };
    let step_z = if diff.z >= -eps { 1.0 } else { 0.0 };

    let same_plane = (step_x + step_y + step_z) == 1.0;

    if same_plane 
    {
        integration = integrate_trapezoid(l1, l2, 0.0, d_dist);
    } 
    else {
        let plane_n = rd.cross(ld).normalize();

        let mut sld = Vec3::new(
            if ld.x == 0.0 { 0.0 } else { ld.x.signum() },
            if ld.y == 0.0 { 0.0 } else { ld.y.signum() },
            if ld.z == 0.0 { 0.0 } else { ld.z.signum() },
        );

        // Replacing null component with 1.0 (if it exists)
        sld += Vec3::ONE - sld.abs();

        let center = (bmax + bmin) * 0.5;
        let extent = (bmax - bmin) * 0.5;

        let c = [
            center + sld * extent,
            center + sld * Vec3::new(-1.0, 1.0, 1.0) * extent,
            center + sld * Vec3::new(1.0, -1.0, 1.0) * extent,
            center + sld * Vec3::new(1.0, 1.0, -1.0) * extent,
        ];

        let mut count = 0;
        let mut hits = [Vec2::ZERO; 2];

        for i in 0..3 {
            // Testing intersections with the 3 edges
            if let Some(h) = intersect_plane_segment(p1, plane_n, c[0], c[i + 1]) {
                let uv1 = coord_on_plane(h, p1, rd, ld);
                let uv2 = coord_on_plane(h, p2, -rd, ld);

                // Checking if the point exists in the surface limited by (p1, p2, ld)
                if uv1.x > 0.0 && uv1.y > 0.0 && uv2.x > 0.0 && uv2.y > 0.0 {
                    if count < 2 {
                        hits[count] = uv1;
                        count += 1;
                    }
                }
            }
        }

        if count == 1 {
            integration = integrate_trapezoid(l1, hits[0].y, 0.0, hits[0].x)
                + integrate_trapezoid(hits[0].y, l2, hits[0].x, d_dist);
        }

        if count == 2 {
            if hits[0].x > hits[1].x {
                // Swap with the hit with min distance
                hits.swap(0, 1);
            }

            integration = integrate_trapezoid(l1, hits[0].y, 0.0, hits[0].x)
                + integrate_trapezoid(hits[0].y, hits[1].y, hits[0].x, hits[1].x)
                + integrate_trapezoid(hits[1].y, l2, hits[1].x, d_dist);
        }
    }

    return integration;
}

//Rendering ///////////////////////////////////////////////////////////////////////////////////////////
fn render_box_volume(ro: Vec3, rd: Vec3, ld: Vec3, lc: Vec3, bmin: Vec3, bmax: Vec3) -> Vec3{

    let light_energy = integrate_box(ro, rd, ld, bmin, bmax);
    let p = phase(rd.dot(ld));
    
    return light_energy * p * lc;
}

pub fn main(frag_coord: Vec2, resolution: Vec2, time: f32, _frame: u32) -> Vec4 {

    let uv = (frag_coord * 2.0 - resolution) / resolution.y;
    let mut final_color = vec3(0.0, 0.0, 0.0);
    
    let cam_pos = vec3((time*0.4).cos() * 6.0, 2.0, (time*0.4).sin() * 6.0);
    let cam_focus = vec3(0.0, 0.0, 0.0);
    
    let cam_for = (cam_focus - cam_pos).normalize();
    let cam_right = cam_for.cross(vec3(0.0, 1.0, 0.0)).normalize();
    let cam_up = cam_right.cross(cam_for).normalize();
    
    let ray_dir = (uv.x * cam_right + uv.y * cam_up + cam_for * 2.0).normalize();
    
    //Light
    let light_dir = (vec3((time + 2.8).cos(), 0.1, (time + 2.8).sin())).normalize();
    let light_color = vec3(1.0, 1.0, 1.0) * 2.0;
    
    //Volume
    let bmin = vec3(-1.0, -1.0, -1.0);
    let bmax = vec3(1.0, 1.0,1.0);
    let box_inter = intersect_aabb(cam_pos, ray_dir, bmin, bmax);
    
    let sun_att = 1.0 - (light_dir - ray_dir).length();
    let sun = smoothstep(0.95, 0.96, sun_att) + sun_att.clamp(0.0, 1.0).powf(15.0);
    
    final_color += sun * 0.35 * light_color;
    
    if box_inter.x != 0.0 && box_inter.y != 0.0
    {
        final_color += render_box_volume(cam_pos, ray_dir, light_dir, light_color, bmin, bmax);
    }

    return vec4(final_color.x, final_color.y, final_color.z, 1.0);
}