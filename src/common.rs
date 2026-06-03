pub use std::f32::consts::PI;

pub use glam::{Vec2, Vec3, Vec4};
pub use glam::{vec2, vec3, vec4};

pub type ShaderFn = fn(frag_coord: Vec2, resolution: Vec2, time: f32, frame: u32) -> Vec4;

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

pub fn color_to_u32(color: Vec4) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0) as u32;
    let a = (color.w.clamp(0.0, 1.0) * 255.0) as u32;

    (a << 24) | (r << 16) | (g << 8) | b
}

pub fn tonemap(color: Vec4) -> Vec4{
    return vec4(color.x.tanh(), color.y.tanh(), color.z.tanh(), color.w);               
}