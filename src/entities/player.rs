use avian3d::prelude::ShapeCaster;
use bevy::{math::Vec3, prelude::{Bundle, Component}};

pub static CAMERA_LOOK_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);
pub static CAMERA_MIN_OFFSET_TRANSLATION: Vec3 = Vec3::new(0.0, 5.0, 5.);
pub static CAMERA_MAX_OFFSET_TRANSLATION: Vec3 = Vec3::new(0.0, 75.0, 100.);

#[derive(Component, Default)]
pub struct PlayerCamera {
    pub location: Vec3,
    pub offset: Vec3,
    pub rotation: Vec3,
    pub zoom: f32,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player_camera: PlayerCamera,
    pub selection_cast: ShapeCaster,
}