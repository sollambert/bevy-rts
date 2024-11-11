use avian3d::prelude::*;
use core::f32;
use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::RenderTarget, window::*};
use bevy_mod_picking::{pointer::*, prelude::*, PointerBundle};

use crate::controls::camera::PlayerCamera;

pub const CURSOR_POSITION_DEFAULT: Vec2 = Vec2::new(0.5, 0.5);
pub const MOUSE_SENSITIVITY: f32 = 10.;

#[derive(Clone, Copy)]
pub struct CursorTextureIndex;

#[allow(unused)]
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
    pub const RESIZE_TB: usize = 10;
    pub const RESIZE_RL: usize = 11;
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
pub struct SelectionBundle {
    pub collider: Collider,
    pub selection: Selection,
}

#[derive(Component, Default)]
pub struct Selection;

#[derive(Bundle, Default)]
pub struct CursorBundle {
    pub cursor: Cursor,
    pub selection: CursorSelection,
}

#[derive(Component, Default)]
pub struct CursorSelection {
    pub start: Option<Vec2>,
}

#[derive(Component, Default)]
pub struct CursorTexture;

#[derive(Event)]
pub struct CursorModeChangeEvent(pub CursorMode);

impl CursorModeChangeEvent {
    pub fn cursor_mode(&self) -> CursorMode {
        return self.0;
    }
}

#[derive(Component, Default)]
pub struct Cursor {
    pub visibility: Visibility,
    pub location: Vec2,
    pub mode: CursorMode,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CursorMode {
    CameraControl,
    #[default]
    Idle,
    Selecting,
    _Locked,
}

impl std::fmt::Display for CursorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn add_cursor_systems(app: &mut App) {
    app
        .add_systems(PostStartup, setup_cursor)
        .add_systems(Update, handle_cursor)
        .add_systems(Update, handle_cursor_mode_event)
        .add_systems(Update, handle_input_press);
}

pub fn setup_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    q_mouse_pointer: Query<(&mut PointerId, Entity)>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut window = q_windows.single_mut();
    window.cursor.visible = false;
    let cursor_position = window.size() * CURSOR_POSITION_DEFAULT;
    
    let (mouse_pointer, mouse_pointer_entity) = q_mouse_pointer.single();
    if mouse_pointer.is_mouse() {
        commands.entity(mouse_pointer_entity).despawn();
    }

    let texture: Handle<Image> = asset_server.load("textures/cursor.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 8, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    window.set_cursor_position(Some(cursor_position));
    
    commands.spawn((
        PointerBundle::new(PointerId::Custom(Uuid::new_v4())),
        CursorBundle {
            cursor: Cursor {
                visibility: Visibility::Visible,
                location: CURSOR_POSITION_DEFAULT,
                ..default()
            },
            ..default()
        }
    ));
    commands.spawn((
        Pickable {
            should_block_lower: false,
            is_hoverable: false,
        },
        NodeBundle {
            style: Style {
                height: Val::Vh(100.),
                width: Val::Vw(100.),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
    ).with_children(|parent| {
        parent.spawn((
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
            CursorTexture,
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
                index: CursorTextureIndex::POINTER,
            }
        ));
    });
}

// Trigger buffered input press events for mapping mouse pointer events to custom pointer
pub fn handle_input_press(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_pointer: Query<&PointerId, With<Cursor>>,
    mut ev_input: EventWriter<InputPress>
) {
    let to_pointer_button = |mouse_button: MouseButton| -> Option<PointerButton> {
        match mouse_button {
            MouseButton::Left => Some(PointerButton::Primary),
            MouseButton::Right => Some(PointerButton::Secondary),
            MouseButton::Middle => Some(PointerButton::Middle),
            _ => None
        }
    };
    let pointer_id = *q_pointer.single_mut();
    for pressed in mouse.get_just_pressed() {
        if let Some(button) = to_pointer_button(*pressed) {
            let input_event = InputPress {
                pointer_id,
                button,
                direction: PressDirection::Down,
            };
            ev_input.send(input_event);
        }
    }
    for released in mouse.get_just_released() {
        if let Some(button) = to_pointer_button(*released) {
            let input_event = InputPress {
                pointer_id,
                button,
                direction: PressDirection::Up,
            };
            ev_input.send(input_event);
        }
    }
}

pub fn handle_cursor(
    time: Res<Time>,
    mut commands: Commands,
    mut q_windows: Query<(&mut Window, Entity)>,
    mut q_camera: Query<(&mut PlayerCamera, &mut Camera), Without<Cursor>>,
    mut q_pointer: Query<(&PointerId, &mut PointerLocation), With<Cursor>>,
    mut q_cursor: Query<(&mut Cursor, Entity)>,
    mut q_cursor_texture_entity: Query<Entity, With<CursorTexture>>,
    mut ev_mouse: EventReader<MouseMotion>,
    mut ev_cursor_change: EventWriter<CursorModeChangeEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let (mut window, window_entity) = q_windows.single_mut();
    let (_camera, _camera_3d) = q_camera.single_mut();
    let (mut cursor, _cursor_entity) = q_cursor.single_mut();
    let cursor_texture_entity = q_cursor_texture_entity.single_mut();
    let delta = time.delta_seconds();

    commands.entity(cursor_texture_entity).insert(
        (cursor.visibility,
        Style {
            position_type: PositionType::Absolute,
            left: Val::Px(cursor.location.x),
            top:  Val::Px(cursor.location.y),
            height: Val::Vw(2.5),
            width: Val::Vw(2.5),
            ..default()
        }
    ));

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }

    if key.just_pressed(KeyCode::AltLeft) {
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    if window.focused && cursor.mode == CursorMode::Idle || cursor.mode == CursorMode::Selecting { 
        for mouse_event in ev_mouse.read() {
            let motion = mouse_event.delta * delta;
            cursor.location += motion * MOUSE_SENSITIVITY;
            cursor.location = cursor.location.clamp(Vec2::ZERO, window.size());
        }
    }

    if let Ok((_pointer_id, mut pointer_location)) = q_pointer.get_single_mut() {
        pointer_location.location = Some(Location {
            position:
                Vec2::new( 
                    cursor.location.x,
                    cursor.location.y
                ),
            target: RenderTarget::Window(WindowRef::Primary)
                .normalize(Some(window_entity))
                .unwrap()
        });
    }

    // Handle state change
    match cursor.mode {
        CursorMode::Idle => {
            if mouse.just_pressed(MouseButton::Left) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::Selecting));
            }
            if mouse.just_pressed(MouseButton::Right) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::CameraControl));
            }
        },
        CursorMode::CameraControl => {
            if mouse.just_released(MouseButton::Right) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::Idle));
            }
        },
        CursorMode::Selecting => {
            if mouse.just_released(MouseButton::Left) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::Idle));
            }
        },
        _ => {
            return;
        }
    }
}

pub fn handle_cursor_mode_event(
    mut q_cursor: Query<&mut Cursor>,
    mut q_cursor_texture: Query<&mut TextureAtlas, With<CursorTexture>>,
    mut ev_cursor_change: EventReader<CursorModeChangeEvent>
) {
    let mut cursor = q_cursor.single_mut();
    let mut texture_atlas = q_cursor_texture.single_mut();
    for cursor_change_event in ev_cursor_change.read() {
        match cursor_change_event.cursor_mode() {
            CursorMode::CameraControl => {
                texture_atlas.index = CursorTextureIndex::CROSSHAIR_5;
            },
            CursorMode::Idle => {
                texture_atlas.index = CursorTextureIndex::POINTER;
            },
            CursorMode::_Locked => {
                texture_atlas.index = CursorTextureIndex::POINTER_X;
            },
            CursorMode::Selecting => {
                texture_atlas.index = CursorTextureIndex::CROSSHAIR_10;
            }
        }
        cursor.mode = cursor_change_event.cursor_mode();
    }
}