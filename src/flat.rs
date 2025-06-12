use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub fn plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<FlatMaterial>::default());
}

/// A simple material to render the texture to an object with color.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlatMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,
    #[uniform(2)]
    pub color: LinearRgba,
}

impl Material for FlatMaterial {
    fn fragment_shader() -> ShaderRef {
        "flat.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
