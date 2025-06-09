use crate::display::RenderTex;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;

#[derive(Component)]
struct UICamera;

pub fn plugin(app: &mut App) {
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
        Msaa::Off,
        RenderLayers::layer(0),
        UICamera,
    ));
    commands.spawn((
        Mesh2d::from(meshes.add(Rectangle::default())),
        MeshMaterial2d::from(materials.add(ColorMaterial {
            color: Color::srgba_u8(255, 0, 0, 255),
            ..default()
        })),
    ));
}
