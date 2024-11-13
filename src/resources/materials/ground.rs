#![allow(unused)]
use bevy::math::Vec2;
use bevy_ambient_cg::ambient_cg::{AmbientCGMaterial, AmbientCGResolution};

pub const GROUND_054: AmbientCGMaterial = AmbientCGMaterial {
    name: "Ground054",
    subfolder: Some("ground"),
    resolution: AmbientCGResolution::OneK,
    uv_scale: Some(Vec2::new(8., 8.))
};