use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::controls::selection::Selectable;

pub mod world_objects;

#[derive(Copy, Clone, PhysicsLayer)]
pub enum EntityCollisionLayers {
    Ground,
    Interaction,
    Selectable,
}

#[derive(Bundle)]
pub struct SelectableActorBundle {
    avian_pickable: AvianPickable,
    collider: Collider,
    collision_layers: CollisionLayers,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    pickable_bundle: PickableBundle,
    rigid_body: RigidBody,
    selectable: Selectable,
    transform: Transform,
}

impl Default for SelectableActorBundle {
    fn default() -> Self {
        Self {
            avian_pickable: AvianPickable,
            collider: Collider::default(),
            collision_layers: CollisionLayers::new(EntityCollisionLayers::Selectable, LayerMask::ALL),
            material: Handle::default(),
            mesh: Handle::default(),
            pickable_bundle: PickableBundle::default(),
            rigid_body: RigidBody::Static,
            selectable: Selectable::default(),
            transform: Transform::default(),
        }
    }
}