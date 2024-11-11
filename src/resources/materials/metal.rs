#![allow(unused)]
use bevy::math::Vec2;

use crate::utils::assets::*;

pub const METAL_055_A: AmbientCGMaterial = AmbientCGMaterial {
    name: "Metal055A",
    resolution: AmbientCGResolution::OneK,
    subfolder: Some("metal"),
    uv_scale: Some(Vec2::new(4., 4.))
};