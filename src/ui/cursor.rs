use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::RenderTarget, window::*};
use bevy_mod_picking::{pointer::{InputPress, Location, PressDirection, Uuid}, prelude::{Pickable, PointerButton, PointerId, PointerLocation}, PointerBundle};

use crate::entities::player::PlayerCamera;

pub const CURSOR_POSITION_DEFAULT: Vec2 = Vec2::new(0.5, 0.5);
pub const MOUSE_SENSITIVITY: f32 = 0.2;

#[derive(Clone, Copy)]
pub struct CursorTextureIndex;

impl CursorTextureIndex {
    pub const Pointer: usize = 0;
    pub const PointerQuestion: usize = 1;
    pub const PointerExclamation: usize = 2;
    pub const PointerCircle: usize = 3;
    pub const PointerX: usize = 4;
    pub const PointerClock: usize = 5;
    pub const PointerBlock: usize = 6;
    pub const PointerFace: usize = 7;
    pub const ResizeTlBr: usize = 8;
    pub const ResizeTrBl: usize = 9;
    pub const ResizeTB: usize = 10;
    pub const ResizeLR: usize = 11;
    pub const ResizeCross: usize = 12;
    pub const Insert: usize = 13;
    pub const Crosshair1: usize = 16;
    pub const Crosshair2: usize = 17;
    pub const Crosshair3: usize = 18;
    pub const Crosshair4: usize = 19;
    pub const Crosshair5: usize = 20;
    pub const Crosshair6: usize = 24;
    pub const Crosshair7: usize = 25;
    pub const Crosshair8: usize = 26;
    pub const Crosshair9: usize = 27;
    pub const Crosshair10: usize = 28;
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

#[derive(Component, Default)]
pub struct CursorTexture;

#[derive(Event)]
pub struct CursorSelectionEvent(pub Rect);

impl CursorSelectionEvent {
    pub fn rect(&self) -> Rect{
        return self.0;
    }
}

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


#[derive(Clone, Copy, Default, PartialEq)]
pub enum CursorMode {
    CameraControl,
    #[default]
    Idle,
    Selecting,
    Locked,
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
                index: CursorTextureIndex::Pointer,
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
    mut q_cursor: Query<(&mut Cursor, &mut CursorSelection, Entity)>,
    mut q_cursor_texture_entity: Query<Entity, With<CursorTexture>>,
    mut ev_mouse: EventReader<MouseMotion>,
    mut ev_cursor_change: EventWriter<CursorModeChangeEvent>,
    mut ev_selection: EventWriter<CursorSelectionEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let (mut window, mut window_entity) = q_windows.single_mut();
    let (camera, camera_3d) = q_camera.single_mut();
    let (mut cursor, mut cursor_selection, entity) = q_cursor.single_mut();
    let mut cursor_texture_entity = q_cursor_texture_entity.single_mut();
    let delta = time.delta_seconds();

    commands.entity(cursor_texture_entity).insert(
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

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }

    if key.just_pressed(KeyCode::AltLeft) {
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    if cursor.mode == CursorMode::Idle || cursor.mode == CursorMode::Selecting { 
        for mouse_event in ev_mouse.read() {
            let motion = mouse_event.delta * delta;
            cursor.location += motion * MOUSE_SENSITIVITY;
            cursor.location = cursor.location.clamp(Vec2::ZERO, Vec2::ONE);
        }
    }

    if let Ok((pointer_id, mut pointer_location)) = q_pointer.get_single_mut() {
        pointer_location.location = Some(Location {
            position:
                Vec2::new( 
                    cursor.location.x * window.width(),
                    cursor.location.y * window.height()
                ),
            target: RenderTarget::Window(WindowRef::Primary)
                .normalize(Some(window_entity))
                .unwrap()
        });
    }

    // Handle state change
    match cursor.mode {
        CursorMode::Idle => {
            if mouse.just_pressed(MouseButton::Right) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::CameraControl));
                return;
            }
            
            if mouse.just_pressed(MouseButton::Left) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::Selecting));
                cursor_selection.start_pos = Some(cursor.location);
                return;
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
                cursor_selection.end_pos = Some(cursor.location);
                if let (Some(start_pos), Some(end_pos)) = (cursor_selection.start_pos, cursor_selection.end_pos) {
                    ev_selection.send(CursorSelectionEvent(Rect {
                        min: start_pos.min(end_pos),
                        max: start_pos.max(end_pos)
                    }));
                }
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
                texture_atlas.index = CursorTextureIndex::Crosshair5;
            },
            CursorMode::Idle => {
                texture_atlas.index = CursorTextureIndex::Pointer;
            },
            CursorMode::Locked => {
                texture_atlas.index = CursorTextureIndex::PointerX;
            },
            CursorMode::Selecting => {
                texture_atlas.index = CursorTextureIndex::Crosshair10;
            }
        }
        cursor.mode = cursor_change_event.cursor_mode();
    }
}