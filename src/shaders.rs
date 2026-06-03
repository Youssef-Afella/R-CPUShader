use crate::common::*;

pub mod sphere_tracing;
pub mod glass_disks;
pub mod volumetric_cube;

pub enum ShaderType{
    SphereTracing,
    GlassDisks,
    VolumetricCube,
}

pub fn get(name: ShaderType) -> Option<ShaderFn> {
    match name {
        ShaderType::SphereTracing => Some(sphere_tracing::main),
        ShaderType::GlassDisks => Some(glass_disks::main),
        ShaderType::VolumetricCube => Some(volumetric_cube::main),
        _          => None,
    }
}