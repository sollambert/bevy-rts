use std::process::exit;

use bevy::{prelude::*, window::{PrimaryWindow, WindowMode}};

use crate::utils::debug::DebugDisplay;

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

pub fn handle_key_window_functions(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut primary_window = q_windows.single_mut();
    let input_map = InputMap::default();

    if key.just_pressed(input_map.close) {
        exit(0);
    }

    if key.just_pressed(input_map.fullscreen) {
        match primary_window.mode {
            WindowMode::Windowed => {
                primary_window.mode = WindowMode::BorderlessFullscreen;
            },
            WindowMode::BorderlessFullscreen => {
                primary_window.mode = WindowMode::Windowed;
            },
            _ => {
                primary_window.mode = WindowMode::Windowed;
            }
        }
    }
}

pub fn handle_debug_keys(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut q_debug_menu: Query<(Entity, &mut DebugDisplay)>,
) {
    let input_map = InputMap::default();
    let (debug_menu_entity, mut debug_display) = q_debug_menu.single_mut();

    if key.just_pressed(input_map.debug_menu) {
        let mut visibility: Visibility = Visibility::Visible;
        let mut debug_menu_commands = commands.entity(debug_menu_entity);
        if debug_display.visibility == Visibility::Visible {
            visibility = Visibility::Hidden;
        }
        debug_display.visibility = visibility;
        debug_menu_commands.insert(NodeBundle {
            visibility,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        });
    }
}