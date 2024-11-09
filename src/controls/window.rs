use bevy::{prelude::*, window::{PrimaryWindow, WindowMode}};

use super::InputMap;

pub fn handle_key_window_functions(
    mut app_exit_events: EventWriter<AppExit>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut primary_window = q_windows.single_mut();
    let input_map = InputMap::default();

    if key.just_pressed(input_map.close) {
        app_exit_events.send(AppExit::Success);
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