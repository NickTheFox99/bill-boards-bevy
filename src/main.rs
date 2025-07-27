mod billboard;
mod cube;
mod display;
mod flat;
mod grid;
mod player;
mod ui;
mod smile;
mod physic_objects;
mod wyatt;

use bevy::color::palettes::css::WHITE;
use crate::flat::{DynamicMaterial, FlatMaterial};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::window::{
    CursorGrabMode, EnabledButtons, PresentMode, PrimaryWindow, WindowMode, WindowTheme,
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_fix_cursor_unlock_web::prelude::*;
use bitflags::bitflags;

#[derive(Resource)]
struct GameSize(Extent3d);

#[derive(Resource)]
struct GameSettings(u32);

bitflags! {
    impl GameSettings: u32 {
        const COLOR_QUANTIZE = 1 << 0;
        const FLAT = 1 << 1;
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings::COLOR_QUANTIZE |
        GameSettings::FLAT
        // Self::empty()
    }
}

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
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            FixPointerUnlockPlugin,
            billboard::plugin,
            player::plugin,
            display::plugin,
            cube::plugin,
            wyatt::plugin,
            ui::plugin,
            flat::plugin,
            smile::plugin,
        ))
        .insert_resource(GameSize(Extent3d {
            width: 320,
            height: 240,
            depth_or_array_layers: 1,
        }))
        // .insert_resource(AmbientLight::NONE)
        .init_resource::<GameSettings>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (toggle_grab_cursor).run_if(input_just_pressed(KeyCode::Tab)),
                fullscreen.run_if(input_just_pressed(KeyCode::F11)),
                quit_handler.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .run();
}

fn setup(
    game_settings: Res<GameSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- GROUND SETUP ---

    commands.spawn((
        Mesh3d(meshes.add(grid::gen_mesh(10))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE.into(),
            alpha_mode: AlphaMode::Opaque,
            unlit: true,
            ..default()
        })),
        DynamicMaterial,
        Transform::from_scale(Vec3::splat(10.0)),
    ));

    if(game_settings.contains(GameSettings::FLAT)) {
        commands.insert_resource(AmbientLight {
            color: WHITE.into(),
            brightness: 4000.0,
            ..default()
        });
    }
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
