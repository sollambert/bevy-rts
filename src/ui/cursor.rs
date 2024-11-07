use std::f32::consts::E;

use bevy::{input::mouse::MouseMotion, prelude::*, window::*};

use crate::entities::player::PlayerCamera;

pub const CURSOR_POSITION_DEFAULT: Vec2 = Vec2::new(0.5, 0.5);
pub const MOUSE_SENSITIVITY: f32 = 0.2;
pub const SCROLL_SPEED: f32 = 50.0;

#[derive(Clone, Copy)]
pub struct CursorTextureIndex(pub usize);

impl CursorTextureIndex {
    pub const POINTER: usize = 0;
    pub const POINTER_QUESTION: usize = 1;
    pub const POINTER_EXCLAMATION: usize = 2;
    pub const POINTER_CIRCLE: usize = 3;
    pub const POINTER_X: usize = 4;
    pub const POINTER_CLOCK: usize = 5;
    pub const POINTER_BLOCK: usize = 6;
    pub const POINTER_FACE: usize = 7;
    pub const RESIZE_TL_BR: usize = 8;
    pub const RESIZE_TR_BL: usize = 9;
    pub const RESIZE_T_B: usize = 10;
    pub const RESIZE_L_R: usize = 11;
    pub const RESIZE_CROSS: usize = 12;
    pub const INSERT: usize = 13;
    pub const CROSSHAIR_1: usize = 16;
    pub const CROSSHAIR_2: usize = 17;
    pub const CROSSHAIR_3: usize = 18;
    pub const CROSSHAIR_4: usize = 19;
    pub const CROSSHAIR_5: usize = 20;
    pub const CROSSHAIR_6: usize = 24;
    pub const CROSSHAIR_7: usize = 25;
    pub const CROSSHAIR_8: usize = 26;
    pub const CROSSHAIR_9: usize = 27;
    pub const CROSSHAIR_10: usize = 28;
}

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
pub struct CursorSelectionEvent(pub Rect);

impl CursorSelectionEvent {
    pub fn rect(&self) -> Rect{
        return self.0;
    }
}

#[derive(Event)]
pub struct CursorChangeEvent(pub CursorTextureIndex);

impl CursorChangeEvent {
    pub fn texture_index(&self) -> usize {
        return self.0.0;
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
    mut q_cursor: Query<(&mut Cursor, &mut CursorSelection, Entity, &mut TextureAtlas)>,
    mut ev_mouse: EventReader<MouseMotion>,
    mut ev_cursor_change: EventWriter<CursorChangeEvent>,
    mut ev_selection: EventWriter<CursorSelectionEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = q_windows.single_mut();
    let mut camera = q_camera.single_mut();
    let delta = time.delta_seconds();
    let (mut cursor, mut cursor_selection, entity, mut texture_atlas) = q_cursor.single_mut();

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

    if mouse.just_pressed(MouseButton::Right) {
        ev_cursor_change.send(CursorChangeEvent(CursorTextureIndex(CursorTextureIndex::CROSSHAIR_5)));
    }
    if mouse.just_released(MouseButton::Right) {
        ev_cursor_change.send(CursorChangeEvent(CursorTextureIndex(CursorTextureIndex::POINTER)));
    }

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
            ev_cursor_change.send(CursorChangeEvent(CursorTextureIndex(CursorTextureIndex::CROSSHAIR_9)));
            texture_atlas.index = 28;
            cursor_selection.start_pos = Some(cursor.location);
        }
        
        if mouse.just_released(MouseButton::Left) {
            ev_cursor_change.send(CursorChangeEvent(CursorTextureIndex(CursorTextureIndex::POINTER)));
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

pub fn handle_cursor_change_event(
    mut q_texture_atlas: Query<&mut TextureAtlas, With<Cursor>>,
    mut ev_cursor_change: EventReader<CursorChangeEvent>
) {
    let mut texture_atlas = q_texture_atlas.single_mut();
    for cursor_change_event in ev_cursor_change.read() {
        texture_atlas.index = cursor_change_event.texture_index();
    }
}