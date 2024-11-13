#![allow(unused)]
use bevy::math::Vec2;
use bevy_ambient_cg::ambient_cg::{AmbientCGMaterial, AmbientCGResolution};

pub const TILES_074: AmbientCGMaterial = AmbientCGMaterial {
    name: "Tiles074",
    resolution: AmbientCGResolution::OneK,
    subfolder: Some("tiles"),
    uv_scale: Some(Vec2::new(24., 24.))
};

pub const TILES_107: AmbientCGMaterial = AmbientCGMaterial {
    name: "Tiles107",
    resolution: AmbientCGResolution::OneK,
    subfolder: Some("tiles"),
    uv_scale: Some(Vec2::new(24., 24.))
};