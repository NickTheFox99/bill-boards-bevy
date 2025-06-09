mod billboard;
mod display;
mod grid;
mod player;
mod cube;
mod ui;

use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::window::{
    CursorGrabMode, EnabledButtons, PresentMode, PrimaryWindow, WindowMode, WindowTheme,
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_fix_cursor_unlock_web::prelude::*;

#[derive(Resource)]
struct GameSize(Extent3d);

#[derive(Component)]
#[require(Camera)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game".into(),
                    name: Some("game.app".into()),
                    resolution: (320., 240.).into(),
                    present_mode: PresentMode::Fifo,
                    window_theme: Some(WindowTheme::Light),
                    enabled_buttons: EnabledButtons {
                        minimize: false,
                        maximize: false,
                        close: false,
                    },
                    // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            FixPointerUnlockPlugin,
            billboard::plugin,
            player::plugin,
            display::plugin,
            cube::plugin,
            ui::plugin,
        ))
        .insert_resource(GameSize(Extent3d {
            width: 320,
            height: 240,
            depth_or_array_layers: 1,
        }))
        .insert_resource(AmbientLight::NONE)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (toggle_grab_cursor)
                    .run_if(input_just_pressed(KeyCode::Tab)),
                fullscreen.run_if(input_just_pressed(KeyCode::F11)),
                quit_handler.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // --- BILL SETUP ---

    let bill_smile: Handle<Image> = assets.load_with_settings("bill_smile.png", |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::nearest(),
            ..default()
        }
    });

    commands.spawn((
        billboard::Billboard,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(0.5, 0.5)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(bill_smile.clone()),
            base_color: Color::srgb_u8(0, 255, 0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // --- GROUND SETUP ---

    commands.spawn((
        Mesh3d(meshes.add(grid::gen_mesh(10))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(10.0)),
    ));
}

fn toggle_grab_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut().unwrap();
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
}

fn fullscreen(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut().unwrap();
    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
}

fn quit_handler(mut exit_writer: EventWriter<AppExit>) {
    exit_writer.write(AppExit::Success);
}
