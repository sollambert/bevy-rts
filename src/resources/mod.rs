use bevy::app::*;
use player::Player;
use selection::{setup_selection_resource, Selection};

pub mod materials;
pub mod player;
pub mod selection;
pub mod settings;

pub fn initialize_resources(app: &mut App) {
    app
        .init_resource::<Player>()
        .init_resource::<Selection>()
        .add_systems(Startup, setup_selection_resource);
}