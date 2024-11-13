use bevy::prelude::{KeyCode, Resource};

pub mod camera;
pub mod selection;
pub mod window;


#[derive(Resource)]
pub struct InputMap {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub turn_r: KeyCode,
    pub turn_l: KeyCode,
    pub close: KeyCode,
    pub fullscreen: KeyCode,
    pub debug_menu: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        return Self {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            turn_r: KeyCode::KeyQ,
            turn_l: KeyCode::KeyE,
            close: KeyCode::Escape,
            fullscreen: KeyCode::F11,

            // debug keys
            debug_menu: KeyCode::F3,
        }
    }
}