use avian3d::prelude::PhysicsLayer;

pub mod player;
pub mod world_objects;

#[derive(Copy, Clone, PhysicsLayer)]
pub enum EntityCollisionLayers {
    Ground,
    Interaction,
    Selectable,
}