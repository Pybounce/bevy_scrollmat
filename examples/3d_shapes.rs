use std::f32::consts::PI;

use bevy::{
    color::palettes::basic::SILVER, pbr::ExtendedMaterial, prelude::*, render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    }
};
use bevy_scrollmat::plugin::{ScrollMatExtension, ScrollMatPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
        ))
        .add_plugins(ScrollMatPlugin)   // add the ScrollMatPlugin
        .add_systems(Startup, (setup, setup_instructions))
        .add_systems(
            Update,
            (
                rotate,
                update_scroll
            ),
        )
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scroll_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, ScrollMatExtension>>>,
) {
    let debug_material = scroll_mats.add(
        ExtendedMaterial {
            base: StandardMaterial {
                base_color_texture: Some(images.add(uv_debug_texture())),
                ..default()
                },
                extension: ScrollMatExtension {
                    scroll_speed: Vec2::Y
                }
            }
        );

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Tetrahedron::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Cone::default()),
        meshes.add(ConicalFrustum::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let extrusions = [
        meshes.add(Extrusion::new(Rectangle::default(), 1.)),
        meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
        meshes.add(Extrusion::new(Annulus::default(), 1.)),
        meshes.add(Extrusion::new(Circle::default(), 1.)),
        meshes.add(Extrusion::new(Ellipse::default(), 1.)),
        meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
        meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(debug_material.clone()),
            Transform::from_xyz(
                -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
                2.0,
                Z_EXTENT / 2.,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            Shape,
        ));
    }

    let num_extrusions = extrusions.len();

    for (i, shape) in extrusions.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(debug_material.clone()),
            Transform::from_xyz(
                -EXTRUSION_X_EXTENT / 2.
                    + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
                2.0,
                -Z_EXTENT / 2.,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            Shape,
        ));
    }

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}

fn setup_instructions(
    mut commands: Commands
) {
    commands.spawn((
        Text(format!("Current Scroll: {}\n\nPress LEFT_ARROW to decrement X.\n\nPress RIGHT_ARROW to increment X.\n\nPress DOWN_ARROW to decrement Y.\n\nPress UP_ARROW to increment Y.", Vec2::Y)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

}

fn update_scroll(
    query: Query<&MeshMaterial3d<ExtendedMaterial<StandardMaterial, ScrollMatExtension>>, With<Shape>>,
    mut scroll_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, ScrollMatExtension>>>,
    input: Res<ButtonInput<KeyCode>>,
    mut text: Single<&mut Text>,
) {
    let change;

    if input.just_pressed(KeyCode::ArrowRight) { change = Vec2::X; }
    else if input.just_pressed(KeyCode::ArrowLeft) { change = -Vec2::X; }
    else if input.just_pressed(KeyCode::ArrowUp) { change = Vec2::Y; }
    else if input.just_pressed(KeyCode::ArrowDown) { change = -Vec2::Y; }
    else { return; }

    for mat_handle in &query {
        if let Some(mat) = scroll_mats.get_mut(mat_handle) {
            mat.extension.scroll_speed += change * 0.5;
            text.0 = format!("Current Scroll: {}\n\nPress LEFT_ARROW to decrement X.\n\nPress RIGHT_ARROW to increment X.\n\nPress DOWN_ARROW to decrement Y.\n\nPress UP_ARROW to increment Y.", mat.extension.scroll_speed);
            return;
        }
    }
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

