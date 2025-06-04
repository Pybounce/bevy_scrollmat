# bevy_scrollmat

A simple Extension Material that applies a scrolling effect to the uvs of the material.

## Usage

```rust

    fn main() {
        // add the ScrollMatPlugin to your app.
        App::new()
            .add_plugins((
                DefaultPlugins,
                ScrollMatPlugin,
        )).run();
    }
    // function that spawns a new shape with a scrolling material
    // note that the object below has no texture so scrolling won't be visible. For an example, check the examples directory
    fn spawn_scrolling_shape(
        mut commands: Commands,
        mut scroll_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, ScrollMatExtension>>>,
    ) {
        let handle = scroll_mats.add(
        ExtendedMaterial {
            base: StandardMaterial {
                // any changes to material you'd like here
                ..default()
                },
                extension: ScrollMatExtension { // add the scroll mat extension
                    scroll_speed: Vec2::Y   // sets the scroll speed to (0, 1) per second
                }
            }
        );
        commands.spawn((
            Mesh3d(mesh_handle),    // mesh_handle defined elsewhere to keep things simple
            MeshMaterial3d(handle),
            Transform::default(),
        ));
    }
```

## Compatibility

| Bevy version | `bevy_simpletoon` version |
| :----------- | :------------------------ |
| `0.16`       | `0.1`                     |
