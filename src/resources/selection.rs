use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Selection {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

pub fn setup_selection_resource(
    mut selection: ResMut<Selection>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    selection.material = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Premultiplied,
        base_color: Color::linear_rgba(0., 0., 0., 0.5),
        cull_mode: None,
        double_sided: true,
        unlit: true,
        ..default()
    });
    selection.mesh = meshes.add(Cuboid::new(0., 0., 0.));
}