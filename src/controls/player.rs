use std::f32::consts::E;

use avian3d::math::TAU;
use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, time::Time};

use crate::{entities::player::{PlayerCamera, CAMERA_LOOK_POINT, CAMERA_MAX_OFFSET_TRANSLATION, CAMERA_MIN_OFFSET_TRANSLATION}, ui::cursor::SCROLL_SPEED};

use super::controls::InputMap;

const SCROLL_SENSITIVITY: f32 = 5.0;
const TURN_SPEED: f32 = TAU / 4.;
const MAX_ZOOM: f32 = 10.;
const MIN_ZOOM: f32 = 1.;

pub fn handle_camera_zoom(
    time: Res<Time>,
    mut ev_mouse: EventReader<MouseWheel>,
    mut q_camera: Query<&mut PlayerCamera>
) {
    let mut camera = q_camera.single_mut();
    let delta = time.delta_seconds();
    for mouse_wheel_event in ev_mouse.read() {
        let mouse_movement = -mouse_wheel_event.y * delta * SCROLL_SENSITIVITY;
        if (mouse_movement < 0.0 && camera.zoom >= MIN_ZOOM) || (mouse_movement > 0.0 && camera.zoom <= MAX_ZOOM) {
            camera.zoom = f32::max(MIN_ZOOM, f32::min(camera.zoom, MAX_ZOOM)) + mouse_movement;
            println!("X: {}, Y: {}, Zoom: {}", mouse_wheel_event.x, mouse_wheel_event.y, camera.zoom);
        }
    }
    camera.offset = Quat::from_rotation_y(camera.rotation.y).mul_vec3(CAMERA_MIN_OFFSET_TRANSLATION.lerp(CAMERA_MAX_OFFSET_TRANSLATION, camera.zoom / MAX_ZOOM));
}

pub fn handle_camera_move(
    time: Res<Time>,
    mut ev_mouse: EventReader<MouseMotion>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<&mut PlayerCamera>,
) {
    let input_map = InputMap::default();
    let mut camera = q_camera.single_mut();
    let delta = time.delta_seconds();
    let rotation_quat = Quat::from_rotation_y(camera.rotation.y);
    let mut translation = Vec3::ZERO;
    if mouse_btn.pressed(MouseButton::Right) {
        for mouse_event in ev_mouse.read() {
            let motion = mouse_event.delta * delta * f32::ln(camera.zoom * E);
            let mouse_offset_vec = rotation_quat.mul_vec3(Vec3::new(motion.x, 0.0, motion.y));
            camera.location += mouse_offset_vec;
        }
        if key.pressed(input_map.turn_l) {
            camera.rotation.y -= TAU * TURN_SPEED * delta;
        }
        if key.pressed(input_map.turn_r) {
            camera.rotation.y += TAU * TURN_SPEED * delta;
        }
        if key.pressed(input_map.left) {
            translation += rotation_quat.mul_vec3(Vec3::NEG_X * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }
        if key.pressed(input_map.right) {
            translation += rotation_quat.mul_vec3(Vec3::X * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }
        if key.pressed(input_map.forward) {
            translation += rotation_quat.mul_vec3(Vec3::NEG_Z * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }
        if key.pressed(input_map.backward) {
            translation += rotation_quat.mul_vec3(Vec3::Z * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }
    }
    camera.location += translation;
    camera.rotation %= TAU;
}

pub fn handle_camera_transform(
    mut q_camera: Query<(&mut PlayerCamera, &mut Transform)>
) {
    let (camera, mut camera_transform) = q_camera.single_mut();
    camera_transform.translation =  camera.location + camera.offset;
    camera_transform.look_at(camera.location + CAMERA_LOOK_POINT, Dir3::Y);
}