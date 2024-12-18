use avian3d::{math::*, parry::shape::SharedShape, prelude::*};
use backend::PointerHits;
use bevy::{pbr::{NotShadowCaster, NotShadowReceiver}, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{entities::EntityCollisionLayers, ui::cursor::*};

use super::camera::PlayerCamera;

#[derive(Event)]
pub struct SelectionEvent {
    clear: bool,
    entity: Entity,
}

#[derive(Event)]
pub struct SelectionStartEvent;

#[derive(Event)]
pub struct SelectionEndEvent;

#[derive(Component, Default)]
pub struct Selected;

#[allow(dead_code)]
#[derive(Component, Default)]
pub struct Selectable {
    pub selection_mask: SelectionMask
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct SelectionSet;

#[allow(dead_code)]
#[derive(Default)]
pub enum SelectionMask {
    #[default]
    None = 0b0000_0000,
    Hq = 0b0000_0001,
    Structure = 0b0000_0010,
    UnitPassive = 0b0000_0100,
    UnitMilitant = 0b0000_1000,
}

pub fn add_selection_systems(app: &mut App) {
    app
        .add_event::<SelectionEvent>()
        .add_event::<SelectionStartEvent>()
        .add_systems(Update, (
            handle_selection_event,
            handle_selection_start_event,
            handle_selection,
            handle_selection_collisions
                .after(handle_selection),
            render_selected_entity_aabb,
            render_selection_collider,
        ).in_set(SelectionSet));
}

pub fn handle_selection_collisions(
    mut ev_selection: EventWriter<SelectionEvent>,
    mut q_selectable: Query<Entity, (With<Selectable>, Without<Selected>)>,
    q_selected: Query<Entity, With<Selected>>,
    q_colliding_entities: Query<&CollidingEntities, With<Selection>>,
    q_pointer_multiselect: Query<&PointerMultiselect>,
) {
    let Some(colliding_entities) = q_colliding_entities.iter().next() else {
        return;
    };

    let pointer_multiselect = q_pointer_multiselect.single();

    if !pointer_multiselect.is_pressed {
        for selected_entity in q_selected.iter() {
            if !colliding_entities.contains(&selected_entity) {
                ev_selection.send(SelectionEvent {
                    entity: selected_entity,
                    clear: false,
                });
            };
        }
    }
    
    for colliding_entity in colliding_entities.iter() {
        let Ok(selected_entity) = q_selectable.get_mut(*colliding_entity) else {
            continue;
        };
        if !q_selected.contains(selected_entity) {
            ev_selection.send(SelectionEvent {
                entity: selected_entity,
                clear: false,
            });
        }
    }
}

pub fn handle_selection(
    mut commands: Commands,
    mut q_selection: Query<(Entity, &mut Collider, Mut<Handle<Mesh>>, &mut Transform), With<Selection>>,
    mut q_cursor: Query<(&mut Cursor, &mut CursorSelection)>,
    q_camera: Query<&PlayerCamera>,
    mut ev_pointer_hits: EventReader<PointerHits>,
    q_collision_layers: Query<&CollisionLayers>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ev_selection_start: EventWriter<SelectionStartEvent>,
) {
    let (cursor, mut cursor_selection) = q_cursor.single_mut();
    // Handle selection
    match cursor.mode {
        CursorMode::Idle => {
            if mouse.just_pressed(MouseButton::Left) {
                println!("Sending selection event");
                ev_selection_start.send(SelectionStartEvent);
            }
            if !mouse.pressed(MouseButton::Left) {
                cursor_selection.start = None;
                if let Ok(selection) = q_selection.get_single_mut() {
                    commands.entity(selection.0).despawn();
                }
            }
        },
        CursorMode::Selecting => {
            let camera = q_camera.single();
            for pointer_hits in ev_pointer_hits.read() {
                let Some((entity, hit_data)) = pointer_hits.picks.iter().next() else { continue; };
                let Ok(collision_layers) = q_collision_layers.get(*entity) else { continue; };
                for selection in q_selection.iter_mut() {
                    let (
                        _selection_entity,
                        mut selection_collider,
                        selection_mesh,
                        mut selection_transform
                    ) = selection;
                    if collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground {
                        let (Some(start), Some(position) )= (cursor_selection.start, hit_data.position) else {
                            return;
                        };
                        let (start, end) = (start, position.xz());
                        let rotation = Quat::from_rotation_y(camera.rotation.y);
                        let midpoint = start.midpoint(end);
                        let rot_matrix = Mat2::from_angle(camera.rotation.y);
                        let (rot_start, rot_end) = (rot_matrix.mul_vec2(start), rot_matrix.mul_vec2(end));
                        let pos_dif = Vec2::abs(rot_start - rot_end);
                        meshes.insert(selection_mesh.id(), Cuboid::new(pos_dif.x, 1000.0, pos_dif.y).into());
                        selection_collider.set_shape(SharedShape::cuboid(pos_dif.x / 2., 500.0, pos_dif.y / 2.));
                        *selection_transform = Transform {
                            translation: Vec3::new(midpoint.x, 0.0, midpoint.y),
                            rotation,
                            ..default()
                        }
                    }
                }
            }
        }, 
        _ => {
            return;
        }
    }
}

pub fn handle_selection_event(
    mut commands: Commands,
    mut ev_selection: EventReader<SelectionEvent>,
    q_selected: Query<Entity, With<Selected>>,
) {
    for event in ev_selection.read() {
        if event.clear {
            for selected_entity in q_selected.iter() {
                deselect_entity(&mut commands, selected_entity);
            }
        }
        if q_selected.contains(event.entity) {
            deselect_entity(&mut commands, event.entity);
        } else {
            select_entity(&mut commands, event.entity);
        }
    }
}

pub fn handle_selection_start_event(
    mut commands: Commands,
    mut q_cursor: Query<(&mut PointerMultiselect, &mut CursorSelection)>,
    mut ev_pointer_hits: EventReader<PointerHits>,
    q_collision_layers: Query<&CollisionLayers>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ev_selection: EventWriter<SelectionEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_selection_start: EventReader<SelectionStartEvent>,
) {
    let Some(_) = ev_selection_start.read().next() else { return; };
    println!("Started selection");
    let (pointer_multiselect, mut cursor_selection) = q_cursor.single_mut();

    for pointer_hits in ev_pointer_hits.read() {
        let Some((entity, hit_data)) = pointer_hits.picks.iter().next() else { continue; };
        let Ok(collision_layers) = q_collision_layers.get(*entity) else { continue; };
        if collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground {
            let Some(position) = hit_data.position else {
                continue;
            };
            cursor_selection.start = Some(position.xz());
            commands.spawn((
                Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                },
                SelectionBundle::default(),
                CollisionLayers::from_bits(EntityCollisionLayers::Interaction.to_bits(), EntityCollisionLayers::Selectable.to_bits()),
                PbrBundle {
                    material: materials.add(StandardMaterial {
                        alpha_mode: AlphaMode::Premultiplied,
                        base_color: Color::linear_rgba(0., 0., 0., 0.5),
                        cull_mode: None,
                        diffuse_transmission: 0.5,
                        double_sided: true,
                        ior: 1.0,
                        specular_transmission: 0.5,
                        unlit: true,
                        ..default()
                    }),
                    mesh: meshes.add(Cuboid::new(0., 0., 0.)),
                    ..default()
                },
                NotShadowCaster,
                NotShadowReceiver,        
            ));
            break;
        } else if collision_layers.memberships & EntityCollisionLayers::Selectable == EntityCollisionLayers::Selectable {
            ev_selection.send(SelectionEvent {
                entity: *entity,
                clear: !pointer_multiselect.is_pressed
            });
        }
    }
}

fn select_entity(commands: &mut Commands, entity: Entity) {
    println!("Selected entity: {:?}", entity);
    commands.entity(entity).insert(Selected);
}

fn deselect_entity(commands: &mut Commands, entity: Entity) {
    println!("Deselected entity: {:?}", entity);
    commands.entity(entity).remove::<Selected>();
}

pub fn render_selection_collider(
    colliders: Query<(
        &Collider,
        &Position,
        &Rotation,
    ), With<Selection>>,
    mut gizmos: Gizmos<PhysicsGizmos>,
) {
    for (collider, position, rotation) in &colliders {
        gizmos.draw_collider(
            collider,
            *position,
            *rotation,
            Color::hsla(128., 100.0, 0.5, 0.75)
        );
    }
}

pub fn render_selected_entity_aabb(
    aabbs: Query<(
        Entity,
        &ColliderAabb
    ), With<Selected>>,
    mut gizmos: Gizmos<PhysicsGizmos>,
) {
    for (_entity, aabb) in &aabbs {
        gizmos.cuboid(
            Transform::from_scale(Vector::from(aabb.size()).f32())
                .with_translation(Vector::from(aabb.center()).f32()),
            Color::hsla(0., 100.0, 0.5, 0.75),
        );
    }
}