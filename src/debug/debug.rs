use bevy::prelude::*;
use bevy_mod_picking::prelude::Pickable;

use crate::{controls::InputMap, Game};

#[derive(Component, Default)]
pub struct DebugDisplay {
    pub visibility: Visibility
}

#[derive(Component)]
pub struct KeyPressDebugDisplay;

pub fn add_debug_systems(app: &mut App) {
    app
        .add_systems(Startup, setup_debug_screen)
        .add_systems(Update, handle_debug_keys)
        .add_systems(Update, update_debug_screen);
}


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