use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, RenderTarget};
use bevy::render::view::RenderLayers;
use crate::display::RenderTex;

#[derive(Component)]
struct UICamera;

pub fn plugin(mut app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, render_tex: Res<RenderTex>) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            target: RenderTarget::Image(render_tex.get_handle().into()),
            ..default()
        },
        RenderLayers::none(),
        UICamera,
    ));
    commands.spawn((
        Mesh2d::from(meshes.add(Rectangle::default())),
        MeshMaterial2d::from(materials.add(ColorMaterial {
            color: Color::srgba_u8(0, 0, 0, 0),
            ..default()
        })),
        Transform::from_translation(Vec3::NEG_Z * 1.0),
    ));
}
