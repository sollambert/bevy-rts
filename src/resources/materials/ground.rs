#![allow(unused)]
use bevy::math::Vec2;

use crate::plugins::assets::*;

pub const GROUND_054: AmbientCGMaterial = AmbientCGMaterial {
    name: "Ground054",
    subfolder: Some("ground"),
    resolution: AmbientCGResolution::OneK,
    uv_scale: Some(Vec2::new(8., 8.))
};