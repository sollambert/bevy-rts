use avian3d::{prelude::{AngularVelocity, Collider, CollisionLayers, Friction, LayerMask, PhysicsDebugPlugin, RigidBody}, PhysicsPlugins};
use bevy::{prelude::*, render::mesh::ConeMeshBuilder};
use controls::{controls::{handle_debug_keys, handle_key_window_functions}, player::{handle_camera_move, handle_camera_transform, handle_camera_zoom, handle_selection_event}};
use entities::{player::{PlayerBundle, PlayerCamera}, EntityCollisionLayers};
use ui::cursor::{handle_cursor, handle_cursor_mode_event, setup_cursor, CursorModeChangeEvent, CursorSelectionEvent};
use utils::debug::{setup_debug_screen, update_debug_screen};

mod controls;
mod entities;
mod ui;
mod utils;

fn main() {
    let plugins = (DefaultPlugins,
        PhysicsPlugins::default());
    let mut app = App::new();
    app.add_plugins(plugins);
    app.init_resource::<Game>()
    .add_event::<CursorModeChangeEvent>()
        .add_event::<CursorSelectionEvent>()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_cursor)
        .add_systems(Update, handle_cursor)
        .add_systems(Update, handle_cursor_mode_event)
        .add_systems(Update, handle_selection_event)
        .add_systems(Update, handle_key_window_functions)
        .add_systems(Update, handle_camera_move)
        .add_systems(Update, handle_camera_zoom)
        .add_systems(Update, handle_camera_transform);
    if cfg!(debug_assertions) {
        let debug_plugins = PhysicsDebugPlugin::default();
        app.add_plugins(debug_plugins)
            .add_systems(Startup, setup_debug_screen)
            .add_systems(Update, handle_debug_keys)
            .add_systems(Update, update_debug_screen);
    }
    app.run();
}

#[derive(Resource)]
struct Game {
    dev_mode: bool
}

impl Default for Game {
    fn default() -> Game {
        let mut dev_mode = false;
        if cfg!(debug_assertions) {
            dev_mode = true;
        }
        Game {
            dev_mode
        }
    }
}

fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // // spawn generator
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Generator.glb")),
    //     ..default()
    // });

    commands.spawn((
        PlayerBundle {
            player_camera: PlayerCamera {
                zoom: 5.0,
                ..default()
            },
            ..default()
        }, Camera3dBundle::default()));
    
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(200.0, 0.1),
        CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
        Friction::new(0.5),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(200.0, 0.1)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, -0.05, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 10.0, 10.0),
        CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(10.0, 10.0, 10.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 5.0, -20.0),
            ..default()
        },
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::cone(10.0, 1.0),
        CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
        PbrBundle {
            mesh: meshes.add(ConeMeshBuilder::new(10.0, 1.0, 16)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(20.0, 0.5, -20.0),
            ..default()
        },
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    for _i in 0..10 {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
            AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb_u8(124, 144, 255)),
                transform: Transform::from_xyz(0.0, 4.0, 0.0),
                ..default()
            },
        ));
    }

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}