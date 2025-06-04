
use bevy::{pbr::ExtendedMaterial, prelude::*};
use bevy_scrollmat::plugin::{ScrollMatExtension, ScrollMatPlugin};


fn main() {
        App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ScrollMatPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ScrollMatExtension>>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(standard_materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::srgb_u8(124, 144, 255),
                ..default()
            },
            extension: ScrollMatExtension {
                scroll_speed: Vec2::Y
            }
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
