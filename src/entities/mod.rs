use avian3d::prelude::PhysicsLayer;

pub mod player;
pub mod world_objects;

#[derive(PhysicsLayer)]
pub enum EntityCollisionLayers {
    Ground,
    Interaction,
}