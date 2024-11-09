use avian3d::{math::*, prelude::{AngularVelocity, Collider, CollisionLayers, Friction, LayerMask, PhysicsDebugPlugin, PhysicsGizmos, RigidBody}, PhysicsPlugins};
use bevy::{prelude::*, render::mesh::ConeMeshBuilder};
use bevy_contact_projective_decals::DecalPlugin;
use bevy_mod_picking::{debug::DebugPickingMode, prelude::{AvianBackend, AvianBackendSettings, AvianPickable, Pickable, RaycastBackend}, DefaultPickingPlugins, PickableBundle};
use controls::{camera::{add_camera_systems, PlayerCamera}, selection::{add_selection_systems, Selectable, SelectionMask}, window::handle_key_window_functions};
use entities::EntityCollisionLayers;
use ui::cursor::{add_cursor_systems, CursorModeChangeEvent};
use debug::debug::add_debug_systems;

mod controls;
mod entities;
mod ui;
mod debug;

fn main() {
    let plugins = (
        DefaultPlugins,
        DecalPlugin,
        DefaultPickingPlugins.build()
            .disable::<RaycastBackend>()
            .enable::<AvianBackend>(),
        PhysicsPlugins::default()
    );
    let mut app = App::new();
    app
        .add_plugins(plugins)
        .insert_resource(AvianBackendSettings {
            require_markers: true, // Optional: only needed when you want fine-grained control over which cameras and entities should be used with the Avian picking backend. This is disabled by default, and no marker components are required on cameras or colliders. This resource is inserted by default, you only need to add it if you want to override the default settings.
        });
    app.init_resource::<Game>()
        .add_event::<CursorModeChangeEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_key_window_functions);
    if cfg!(debug_assertions) {
        let debug_plugins = PhysicsDebugPlugin::default();
        app.add_plugins(debug_plugins)
            .insert_resource(DebugPickingMode::Normal);
        add_debug_systems(&mut app);
    } else {
        app.add_plugins(PhysicsDebugPlugin::default())
            .insert_gizmo_config(
                PhysicsGizmos::none(),
                GizmoConfig::default(),
            );
    }
    add_camera_systems(&mut app);
    add_cursor_systems(&mut app);
    add_selection_systems(&mut app);
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
        AvianPickable,
        PlayerCamera {
            zoom: 5.0,
            ..default()
        },
        Camera3dBundle::default()
    ));
    
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        AvianPickable,
        Pickable {
            should_block_lower: true,
            is_hoverable: false,
        },
        Collider::cylinder(200.0, 0.1),
        CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
        Friction::new(0.5),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(200.0, 0.1)),
            material: materials.add(Color::linear_rgb(0.25, 0.25, 0.25)),
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
            AvianPickable,
            PickableBundle {
                pickable: Pickable {
                    should_block_lower: false,
                    is_hoverable: true,
                },
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            CollisionLayers::new(EntityCollisionLayers::Selectable, LayerMask::ALL),
            AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
            Selectable {
                selection_mask: SelectionMask::UnitPassive
            },
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

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::hsl(42.0, 69., 77.),
            illuminance: 1_000.,
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_euler(EulerRot::YXZ, 0.0, -PI / 1.8, 0.0),
            ..default()
        },
        ..default()
    });
}