
use bevy::{asset::embedded_asset, pbr::{ExtendedMaterial, MaterialExtension}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}};

pub struct ScrollMatPlugin;

impl Plugin for ScrollMatPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/scrollmat.wgsl");
        app.add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, ScrollMatExtension>>::default());

    }
}


#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct ScrollMatExtension {
    #[uniform(100)]
    pub scroll_speed: Vec2,
}

impl MaterialExtension for ScrollMatExtension {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_scrollmat/assets/scrollmat.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://bevy_scrollmat/assets/scrollmat.wgsl".into()
    }

}