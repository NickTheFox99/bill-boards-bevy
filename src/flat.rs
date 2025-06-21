use crate::GameSettings;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup, CompareFunction, DepthBiasState, DepthStencilState, RenderPipelineDescriptor,
    ShaderRef, SpecializedMeshPipelineError, StencilState, TextureFormat,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<FlatMaterial>::default());
    app.add_systems(
        PreUpdate,
        set_materials.run_if(resource_changed::<GameSettings>),
    );
}

/// A simple material to render the texture to an object with color.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlatMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,
    #[uniform(2)]
    pub color: LinearRgba,
    pub alpha_mode: AlphaMode,
}

impl Material for FlatMaterial {
    fn fragment_shader() -> ShaderRef {
        "flat.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(pipeline: &MaterialPipeline<Self>, descriptor: &mut RenderPipelineDescriptor, layout: &MeshVertexBufferLayoutRef, key: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        info!("{:?}",descriptor.primitive.cull_mode);
        Ok(())
    }
}

/// A flag component to force the given Material, not shifting with flat / shaded
#[derive(Component)]
pub struct MaterialOverride;

pub fn set_materials(
    mut commands: Commands,
    mut f_mats: ResMut<Assets<FlatMaterial>>,
    mut s_mats: ResMut<Assets<StandardMaterial>>,
    std_entities: Query<(Entity, &MeshMaterial3d<StandardMaterial>), Without<MaterialOverride>>,
    flat_entities: Query<(Entity, &MeshMaterial3d<FlatMaterial>), Without<MaterialOverride>>,
    g_set: Res<GameSettings>,
) {
    if g_set.contains(GameSettings::FLAT) {
        for (e, mat) in std_entities {
            let o_mat = s_mats.get(mat.id()).unwrap();
            let n_mat = MeshMaterial3d(f_mats.add(FlatMaterial {
                color: o_mat.base_color.clone().into(),
                texture: o_mat.base_color_texture.clone(),
                alpha_mode: o_mat.alpha_mode,
            }));
            commands
                .entity(e)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(n_mat)
                .log_components();
        }
    } else {
        for (e, mat) in flat_entities {
            let o_mat = f_mats.get(mat.id()).unwrap();
            let n_mat = MeshMaterial3d(s_mats.add(StandardMaterial {
                base_color: o_mat.color.clone().into(),
                base_color_texture: o_mat.texture.clone(),
                alpha_mode: o_mat.alpha_mode,
                unlit: true,
                ..default()
            }));
            commands
                .entity(e)
                .remove::<MeshMaterial3d<FlatMaterial>>()
                .insert(n_mat)
                .log_components();
        }
    }
}
