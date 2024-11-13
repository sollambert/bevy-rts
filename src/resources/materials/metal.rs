#![allow(unused)]
use bevy::math::Vec2;
use bevy_ambient_cg::ambient_cg::{AmbientCGMaterial, AmbientCGResolution};

pub const METAL_055_A: AmbientCGMaterial = AmbientCGMaterial {
    name: "Metal055A",
    resolution: AmbientCGResolution::OneK,
    subfolder: Some("metal"),
    uv_scale: Some(Vec2::new(4., 4.))
};