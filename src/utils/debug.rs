use bevy::prelude::*;
use bevy_mod_picking::prelude::Pickable;

use crate::Game;

#[derive(Component, Default)]
pub struct DebugDisplay {
    pub visibility: Visibility
}

#[derive(Component)]
pub struct KeyPressDebugDisplay;

pub fn setup_debug_screen(
    mut commands: Commands,
    game: Res<Game>
) {
    let mut visibility = Visibility::Hidden;
    if game.dev_mode {
        visibility = Visibility::Visible;
    }
    commands.spawn((
        Pickable {
            should_block_lower: false,
            is_hoverable: false,
        },
        DebugDisplay {
            visibility,
        },
        NodeBundle {
            visibility,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
    )).with_children(|parent| {
        parent.spawn(KeyPressDebugDisplay);
    });
}

pub fn update_debug_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    key: Res<ButtonInput<KeyCode>>,
    mut q_key_press_debug_display: Query<Entity, With<KeyPressDebugDisplay>>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/Roboto/Roboto-Light.ttf"),
        font_size: 16.0,
        ..default()
    };
    let key_press_debug_display = q_key_press_debug_display.single_mut();

    // Grab pressed keys and build string
    let mut keys = String::new();
    key.get_pressed().for_each(|key | {
        keys += &format!("{:?} ", key);
    });
    keys = keys.trim().to_owned();

    // Create key display
    commands.entity(key_press_debug_display).insert(TextBundle::from_section(
        format!("Keys: {}", keys),
        text_style.to_owned()
    ));
}