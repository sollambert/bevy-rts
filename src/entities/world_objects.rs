use avian3d::prelude::CollisionLayers;
use bevy::prelude::*;
use bevy_mod_picking::backend::PointerHits;

use super::EntityCollisionLayers;

pub fn handle_world_object_drag(
    q_map: Query<(Entity, &CollisionLayers)>,
    q_pointer_hits: Query<&PointerHits>,
) {
    for pointer_hit in q_pointer_hits.iter() {
        for (entity, hit_data) in pointer_hit.picks.iter() {
            if let Ok((map_entity, collision_layers)) = q_map.get(*entity) {
                if collision_layers.memberships & EntityCollisionLayers::Ground == EntityCollisionLayers::Ground{
                    println!("{:?}", hit_data.position);
                }
            }
        }
    }
}