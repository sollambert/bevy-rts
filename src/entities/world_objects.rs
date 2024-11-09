use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_picking::backend::PointerHits;

use crate::ui::cursor::Selection;

#[derive(Component, Default)]
pub struct Selectable;

pub fn handle_world_object_drag(
    _q_map: Query<(Entity, &CollisionLayers)>,
    _q_pointer_hits: Query<&PointerHits>,
) {
    // for pointer_hit in q_pointer_hits.iter() {
    //     for (entity, hit_data) in pointer_hit.picks.iter() {
    //         if let Ok((map_entity, collision_layers)) = q_map.get(*entity) {
    //             if collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground{
    //                 println!("{:?}", hit_data.position);
    //             }
    //         }
    //     }
    // }
}

pub fn handle_selection(
    mut q_selectable: Query<(Entity, &mut Transform, &Handle<StandardMaterial>), With<Selectable>>,
    mut q_colliding_entities: Query<&CollidingEntities, (
        With<Collider>,
        With<Selection>
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(colliding_entities) = q_colliding_entities.get_single_mut() else {
        return;
    };
    
    for colliding_entity in colliding_entities.iter() {
        let Ok((_selected_entity, mut _selected_transform, selected_material)) = q_selectable.get_mut(*colliding_entity) else {
            continue;
        };
        let Some(material) = materials.get_mut(selected_material.id()) else {
            continue;
        };
        material.base_color = Color::WHITE;
    }
}