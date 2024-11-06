use std::f32::consts::E;

use bevy::{input::mouse::MouseMotion, prelude::*, window::*};

use crate::entities::player::PlayerCamera;

pub const CURSOR_POSITION_DEFAULT: Vec2 = Vec2::new(0.5, 0.5);
pub const MOUSE_SENSITIVITY: f32 = 0.2;
pub const SCROLL_SPEED: f32 = 50.0;

#[derive(Bundle, Default)]
pub struct CursorBundle {
    pub cursor: Cursor,
    pub selection: CursorSelection,
}

#[derive(Component, Default)]
pub struct CursorSelection {
    pub start_pos: Option<Vec2>,
    pub end_pos: Option<Vec2>
}

#[derive(Event)]
pub struct CursorSelectionEvent(Rect);

impl CursorSelectionEvent {
    pub fn rect(&self) -> Rect{
        return self.0;
    }
}

#[derive(Component, Default)]
pub struct Cursor {
    pub visibility: Visibility,
    pub location: Vec2,
}

pub fn setup_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut window = q_windows.single_mut();
    window.cursor.visible = false;
    let cursor_position = window.size() * CURSOR_POSITION_DEFAULT;

    let texture: Handle<Image> = asset_server.load("textures/cursor.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 8, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    window.set_cursor_position(Some(cursor_position));
    commands.spawn(
        NodeBundle {
            style: Style {
                height: Val::Vh(100.),
                width: Val::Vw(100.),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn((
            CursorBundle {
                cursor: Cursor {
                    visibility: Visibility::Visible,
                    location: CURSOR_POSITION_DEFAULT,
                },
                ..default()
            },
            ImageBundle {
                image: texture.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(cursor_position.x),
                    top:  Val::Px(cursor_position.y),
                    height: Val::Px(16.),
                    width: Val::Px(16.),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 5,
            }
        ));
    });
}

pub fn handle_cursor(
    time: Res<Time>,
    mut commands: Commands,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut q_camera: Query<&mut PlayerCamera>,
    mut q_cursor: Query<(&mut Cursor, &mut CursorSelection, Entity)>,
    mut ev_mouse: EventReader<MouseMotion>,
    mut ev_selection: EventWriter<CursorSelectionEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = q_windows.single_mut();
    let mut camera = q_camera.single_mut();
    let delta = time.delta_seconds();
    let (mut cursor, mut cursor_selection, entity) = q_cursor.single_mut();

    commands.entity(entity).insert(
        (cursor.visibility,
        Style {
            position_type: PositionType::Absolute,
            left: Val::Px(window.width() * cursor.location.x),
            top:  Val::Px(window.height() * cursor.location.y),
            height: Val::Vw(2.5),
            width: Val::Vw(2.5),
            ..default()
        }
    ));

    let mut translation = Vec3::ZERO;
    let rotation_quat = Quat::from_rotation_y(camera.rotation.y);

    if !mouse.pressed(MouseButton::Right) {
        for mouse_event in ev_mouse.read() {
            let motion = mouse_event.delta * delta;
            cursor.location += motion * MOUSE_SENSITIVITY;
            cursor.location = cursor.location.clamp(Vec2::ZERO, Vec2::ONE);
        }
        if cursor.location.x == 0.0 {
            translation += rotation_quat.mul_vec3(Vec3::NEG_X * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }
        if cursor.location.x == 1.0 {
            translation += rotation_quat.mul_vec3(Vec3::X * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }
        if cursor.location.y == 0.0 {
            translation += rotation_quat.mul_vec3(Vec3::NEG_Z * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }
        if cursor.location.y == 1.0 {
            translation += rotation_quat.mul_vec3(Vec3::Z * SCROLL_SPEED * delta * f32::ln(camera.zoom * E));
        }

        if mouse.just_pressed(MouseButton::Left) {
            cursor_selection.start_pos = Some(cursor.location);
        }
        
        if mouse.just_released(MouseButton::Left) {
            cursor_selection.end_pos = Some(cursor.location);
            if let (Some(start_pos), Some(end_pos)) = (cursor_selection.start_pos, cursor_selection.end_pos) {
                ev_selection.send(CursorSelectionEvent(Rect {
                    min: start_pos.min(end_pos),
                    max: start_pos.max(end_pos)
                }));
            }
        }
    }

    camera.location += translation;

    if mouse.just_pressed(MouseButton::Left) {    
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }

    if key.just_pressed(KeyCode::AltLeft) {    
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

pub fn handle_selection_event(
    mut ev_selection: EventReader<CursorSelectionEvent>
) {
    for selection_event in ev_selection.read() {
        println!("{:?}", selection_event.rect());
    }
}