use avian3d::{parry::shape::SharedShape, prelude::*};
use core::f32;
use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::RenderTarget, window::*};
use bevy_mod_picking::{backend::PointerHits, pointer::*, prelude::*, PointerBundle};

use crate::entities::{player::PlayerCamera, EntityCollisionLayers};

pub const CURSOR_POSITION_DEFAULT: Vec2 = Vec2::new(0.5, 0.5);
pub const MOUSE_SENSITIVITY: f32 = 0.2;

#[derive(Clone, Copy)]
pub struct CursorTextureIndex;

impl CursorTextureIndex {
    pub const POINTER: usize = 0;
    pub const _POINTER_QUESTION: usize = 1;
    pub const _POINTER_EXCLAMATION: usize = 2;
    pub const _POINTER_CIRCLE: usize = 3;
    pub const POINTER_X: usize = 4;
    pub const _POINTER_CLOCK: usize = 5;
    pub const _POINTER_BLOCK: usize = 6;
    pub const _POINTER_FACE: usize = 7;
    pub const _RESIZE_TL_BR: usize = 8;
    pub const _RESIZE_TR_BL: usize = 9;
    pub const _RESIZE_TB: usize = 10;
    pub const _RESIZE_RL: usize = 11;
    pub const _RESIZE_CROSS: usize = 12;
    pub const _INSERT: usize = 13;
    pub const _CROSSHAIR_1: usize = 16;
    pub const _CROSSHAIR_2: usize = 17;
    pub const _CROSSHAIR_3: usize = 18;
    pub const _CROSSHAIR_4: usize = 19;
    pub const CROSSHAIR_5: usize = 20;
    pub const _CROSSHAIR_6: usize = 24;
    pub const _CROSSHAIR_7: usize = 25;
    pub const _CROSSHAIR_8: usize = 26;
    pub const _CROSSHAIR_9: usize = 27;
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
    pub start_pos: Option<Vec3>,
    pub end_pos: Option<Vec3>
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

#[derive(Clone, Copy, Default, PartialEq)]
pub enum CursorMode {
    CameraControl,
    #[default]
    Idle,
    Selecting,
    _Locked,
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
    mut q_selection: Query<(Entity, &mut Collider, Mut<Handle<Mesh>>, &mut Transform), With<Selection>>,
    mut q_cursor: Query<(&mut Cursor, &mut CursorSelection, Entity)>,
    mut q_cursor_texture_entity: Query<Entity, With<CursorTexture>>,
    mut ev_mouse: EventReader<MouseMotion>,
    mut ev_cursor_change: EventWriter<CursorModeChangeEvent>,
    mut ev_pointer_hits: EventReader<PointerHits>,
    q_map: Query<(Entity, &CollisionLayers)>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (mut window, window_entity) = q_windows.single_mut();
    let (_camera, _camera_3d) = q_camera.single_mut();
    let (mut cursor, mut cursor_selection, _cursor_entity) = q_cursor.single_mut();
    let cursor_texture_entity = q_cursor_texture_entity.single_mut();
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

    if let Ok((_pointer_id, mut pointer_location)) = q_pointer.get_single_mut() {
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
                for pointer_hits in ev_pointer_hits.read() {
                    let picks = pointer_hits.picks.iter().next();
                    if let Some((entity, hit_data)) = picks {
                        let Ok((_map_entity, collision_layers)) = q_map.get(*entity) else {
                            continue;
                        };
                        if mouse.just_pressed(MouseButton::Left)
                            && collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground
                        {
                            ev_cursor_change.send(CursorModeChangeEvent(CursorMode::Selecting));
                            cursor_selection.start_pos = hit_data.position;
                            if let Some(position) = hit_data.position {
                                println!("{}", position);
                                commands.spawn((
                                    SelectionBundle::default(),
                                    CollisionLayers::from_bits(EntityCollisionLayers::Interaction.to_bits(), EntityCollisionLayers::Selectable.to_bits()),
                                    PbrBundle {
                                        material: materials.add(Color::linear_rgba(0.75, 0.75, 0.75, 0.5)),
                                        mesh: meshes.add(Cuboid::new(1.0, 1000.0, 1.0)),
                                        ..default()
                                    },
                                ));
                                break;
                            }
                        }
                    }
                }
                return;
            }
        },
        CursorMode::CameraControl => {
            if mouse.just_released(MouseButton::Right) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::Idle));
            }
        },
        CursorMode::Selecting => {
            for pointer_hits in ev_pointer_hits.read() {
                let picks = pointer_hits.picks.iter().next();
                if let Some((entity, hit_data)) = picks {
                    let Ok((_map_entity, collision_layers)) = q_map.get(*entity) else {
                        continue;
                    };
                    let Ok((
                        _selection_entity,
                        mut selection_collider,
                        selection_mesh,
                        mut selection_transform
                    )) = q_selection.get_single_mut() else {
                        break;
                    };
                    if collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground
                    {
                        cursor_selection.end_pos = hit_data.position;
                        let (Some(start_pos), Some(end_pos)) = (cursor_selection.start_pos, cursor_selection.end_pos) else {
                            break;
                        };
                        println!("Start: {:.4}\nEnd:{:.4}", start_pos, end_pos);
                        let pos_dif = Vec3::abs(start_pos - end_pos);
                        let midpoint = start_pos.midpoint(end_pos);
                        meshes.insert(selection_mesh.id(), Cuboid::new(pos_dif.x, 1000.0, pos_dif.z).into());
                        selection_collider.set_shape(SharedShape::cuboid(pos_dif.x / 2., 500.0, pos_dif.z / 2.));
                        *selection_transform = Transform {
                            translation: midpoint,
                            ..default()
                        }
                    }
                }
            }
            if mouse.just_released(MouseButton::Left) {
                ev_cursor_change.send(CursorModeChangeEvent(CursorMode::Idle));
                if let Ok(selection) = q_selection.get_single_mut() {
                    commands.entity(selection.0).despawn();
                }
                return;
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