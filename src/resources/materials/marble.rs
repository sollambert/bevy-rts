#![allow(unused)]
use bevy::math::Vec2;
use bevy_ambient_cg::ambient_cg::{AmbientCGMaterial, AmbientCGResolution};

pub const MARBLE_006: AmbientCGMaterial = AmbientCGMaterial {
    name: "Marble006",
    resolution: AmbientCGResolution::OneK,
    subfolder: Some("marble"),
    uv_scale: Some(Vec2::new(4., 4.))
};