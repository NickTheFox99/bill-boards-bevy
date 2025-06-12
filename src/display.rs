use crate::GameSize;
use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    AsBindGroup, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2d, Material2dPlugin};
use bevy::window::{PrimaryWindow, WindowRef};

#[derive(Resource)]
pub struct RenderTex(Option<Handle<Image>>);

impl RenderTex {
    /// Return a cloned handle to the Render Texture image.
    pub fn get_handle(&self) -> Handle<Image> {
        self.0.clone().unwrap().clone()
    }
}

impl Default for RenderTex {
    fn default() -> Self {
        RenderTex(None)
    }
}

#[derive(Component)]
#[require(Mesh2d)]
struct RenderQuad;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct QuantizerMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Option<Handle<Image>>,
}

impl Material2d for QuantizerMaterial {
    fn fragment_shader() -> ShaderRef {
        "color_quant.wgsl".into()
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<RenderTex>();
    app.add_plugins(Material2dPlugin::<QuantizerMaterial>::default());
    app.add_systems(PreStartup, setup); // Important to get texture before init other resources
    app.add_systems(Update, resize);
}

fn setup(
    mut commands: Commands,
    mut render_tex: ResMut<RenderTex>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_material: ResMut<Assets<QuantizerMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_size: Res<GameSize>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut g_set: ResMut<crate::GameSettings>,
) {
    let window = windows.single().unwrap();

    let mut render_target = Image::new_fill(
        game_size.0,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );

    render_target.texture_descriptor = TextureDescriptor {
        label: None,
        size: game_size.0,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Bgra8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[TextureFormat::Bgra8UnormSrgb],
    };

    render_target.sampler = ImageSampler::nearest();

    render_tex.0 = Some(images.add(render_target));

    commands.spawn((
        Camera2d,
        Camera {
            target: RenderTarget::Window(WindowRef::Primary),
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    let scale = f32::min(
        window.width() / game_size.0.width as f32,
        window.height() / game_size.0.height as f32,
    );

    let mut display = commands.spawn((
        RenderQuad,
        Mesh2d(meshes.add(Rectangle::default())),
        Transform {
            scale: Vec3::new(scale, scale, 0.0),
            rotation: Quat::IDENTITY,
            translation: Vec3::Z * 5.0,
        },
        RenderLayers::layer(1),
    ));
    if cfg!(target_arch = "wasm32") || !g_set.contains(crate::GameSettings::COLOR_QUANTIZE) {
        display.insert(MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(render_tex.get_handle()),
            ..default()
        })));
        info!("Defaulting to ColorMaterial!");
        g_set.set(crate::GameSettings::COLOR_QUANTIZE, false);
    } else {
        display.insert(MeshMaterial2d(q_material.add(QuantizerMaterial {
            texture: Some(render_tex.get_handle()),
        })));
        info!("Quantizer Material picked!");
        g_set.set(crate::GameSettings::COLOR_QUANTIZE, true);
    }
}

fn resize(
    mut quad_query: Query<&mut Transform, With<RenderQuad>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_size: Res<GameSize>,
) {
    let window = window_query.single().unwrap();
    let mut quad = quad_query.single_mut().unwrap();

    let scale = f32::min(
        window.width() / game_size.0.width as f32,
        window.height() / game_size.0.height as f32,
    );

    quad.scale = Vec3::new(
        game_size.0.width as f32 * scale,
        game_size.0.height as f32 * scale,
        1.0,
    );
}
