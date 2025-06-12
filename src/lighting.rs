use crate::GameSettings;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_8;

pub fn plugin(app: &mut App) {
    app.add_systems(PostStartup, setup);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    players: Query<Entity, With<crate::player::Player>>,
    g_set: Res<GameSettings>,
) {
    if g_set.contains(GameSettings::FLAT_LIGHT) {
        return;
    }

    let light = commands
        .spawn(SpotLight {
            intensity: 100_000.0,
            color: WHITE.into(),
            shadows_enabled: false,
            inner_angle: 0.0,
            outer_angle: FRAC_PI_8,
            ..default()
        })
        .id();

    commands
        .entity(players.single().unwrap())
        .insert_children(0, &[light]);
}
