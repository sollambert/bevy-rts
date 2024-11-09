use avian3d::{math::*, parry::shape::SharedShape, prelude::*};
use backend::PointerHits;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{entities::EntityCollisionLayers, ui::cursor::*};

#[derive(Component, Default)]
pub struct Selected;

#[derive(Component, Default)]
pub struct Selectable {
    pub selection_mask: SelectionMask
}

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
        .add_systems(Update, handle_selection)
        .add_systems(Update, handle_selection_collisions)
        .add_systems(Update, render_selected_entity_ring)
        .add_systems(Update, render_selection_aabb);
}

pub fn handle_selection_collisions(
    mut commands: Commands,
    mut q_selectable: Query<Entity, With<Selectable>>,
    mut q_colliding_entities: Query<&CollidingEntities, (
        With<Collider>,
        With<Selection>
    )>,
) {
    let Ok(colliding_entities) = q_colliding_entities.get_single_mut() else {
        return;
    };
    
    for selectable_entity in q_selectable.iter() {
        if colliding_entities.contains(&selectable_entity) {
            continue;
        };
        commands.entity(selectable_entity).remove::<Selected>();
    }
    
    for colliding_entity in colliding_entities.iter() {
        let Ok(selected_entity) = q_selectable.get_mut(*colliding_entity) else {
            continue;
        };
        commands.entity(selected_entity).insert(Selected);
    }
}

pub fn handle_selection(
    mut commands: Commands,
    mut q_selection: Query<(Entity, &mut Collider, Mut<Handle<Mesh>>, &mut Transform), With<Selection>>,
    mut q_cursor: Query<(&mut Cursor, &mut CursorSelection)>,
    mut ev_pointer_hits: EventReader<PointerHits>,
    q_map: Query<(Entity, &CollisionLayers)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (cursor, mut cursor_selection) = q_cursor.single_mut();
    // Handle selection
    match cursor.mode {
        CursorMode::Idle => {
            if !mouse.just_pressed(MouseButton::Left) {
                return;
            }
            for pointer_hits in ev_pointer_hits.read() {
                let Some((entity, hit_data)) = pointer_hits.picks.iter().next() else { continue; };
                let Ok((_map_entity, collision_layers)) = q_map.get(*entity) else { continue; };
                if collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground {
                    cursor_selection.start_pos = hit_data.position;
                    commands.spawn((
                        SelectionBundle::default(),
                        CollisionLayers::from_bits(EntityCollisionLayers::Interaction.to_bits(), EntityCollisionLayers::Selectable.to_bits()),
                        PbrBundle {
                            material: materials.add(StandardMaterial {
                                alpha_mode: AlphaMode::Premultiplied,
                                base_color: Color::linear_rgba(0., 0., 0., 0.25),
                                cull_mode: None,
                                diffuse_transmission: 1.0,
                                double_sided: true,
                                ior: 1.0,
                                specular_transmission: 1.0,
                                unlit: true,
                                ..default()
                            }),
                            mesh: meshes.add(Cuboid::new(1.0, 1000.0, 1.0)),
                            ..default()
                        },
                    ));
                    break;
                }
            }
        },
        CursorMode::Selecting => {
            for pointer_hits in ev_pointer_hits.read() {
                let Some((entity, hit_data)) = pointer_hits.picks.iter().next() else { continue; };
                let Ok((_map_entity, collision_layers)) = q_map.get(*entity) else { continue; };
                let Ok((
                    _selection_entity,
                    mut selection_collider,
                    selection_mesh,
                    mut selection_transform
                )) = q_selection.get_single_mut() else {
                    break;
                };
                if collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground {
                    cursor_selection.end_pos = hit_data.position;
                    let (Some(start_pos), Some(end_pos)) = (cursor_selection.start_pos, cursor_selection.end_pos) else { break; };
                    let (min, max) = (start_pos.min(end_pos), start_pos.max(end_pos));
                    let pos_dif = Vec3::abs(max - min);
                    let midpoint = min.midpoint(max);
                    meshes.insert(selection_mesh.id(), Cuboid::new(pos_dif.x, 1000.0, pos_dif.z).into());
                    selection_collider.set_shape(SharedShape::cuboid(pos_dif.x / 2., 500.0, pos_dif.z / 2.));
                    *selection_transform = Transform {
                        translation: midpoint,
                        ..default()
                    }
                }
            }
            if mouse.just_released(MouseButton::Left) {
                if let Ok(selection) = q_selection.get_single_mut() {
                    commands.entity(selection.0).despawn();
                }
            }
        }, 
        _ => {
            return;
        }
    }
}

pub fn render_selection_aabb(
    aabbs: Query<(
        Entity,
        &ColliderAabb
    ), With<Selection>>,
    mut gizmos: Gizmos<PhysicsGizmos>,
) {
    for (_entity, aabb) in &aabbs {
        gizmos.cuboid(
            Transform::from_scale(Vector::from(aabb.size()).f32())
                .with_translation(Vector::from(aabb.center()).f32()),
            Color::hsla(128., 100.0, 0.5, 0.75),
        );
    }
}

pub fn render_selected_entity_ring(
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