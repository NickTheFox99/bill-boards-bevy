// lawson's a pookie

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}

#[derive(Component)]
struct Lawson {
    y: f32
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("lawson.glb"))),
        Lawson {
            y: 0.7,
        },
        Transform::from_scale(Vec3::splat(1.0/5.0)).with_translation(Vec3::new(-5.0,0.7,2.0)),
        crate::sinphase::SinPhase::new(0.25),
    ));
}

fn update(
    player_pos: Query<&Transform, (With<crate::player::Player>, Without<Lawson>)>,
    time: Res<Time>,
    mut wyatt_pos: Query<(&mut Transform, &Lawson, &crate::sinphase::SinPhase)>,
) {
    let (mut lawson_pos, lawson, sinphase) = wyatt_pos.single_mut().unwrap();
    let player = player_pos.single().unwrap();

    let angle_to_player = Transform::from_translation(lawson_pos.translation)
        .looking_at(player.translation, Dir3::Y)
        .rotation.to_euler(EulerRot::YXZ).0;

    let current_quat = lawson_pos.rotation;
    let target_quat = Quat::from_euler(EulerRot::YXZ, angle_to_player, 0.0, 0.0);
    let new_rotation = current_quat.lerp(target_quat, (33.0 * time.delta_secs()).clamp(0.0, 1.0));

    lawson_pos.rotation = new_rotation;

    let dist = lawson_pos.translation.distance_squared(player.translation);

    let f = lawson_pos.forward();
    lawson_pos.translation += f * time.delta_secs() * (dist - 0.1) * 1.4;
    lawson_pos.translation.y = lawson.y + (sinphase.get_phase() / 100.0) * (dist - 0.1125).clamp(0.0, f32::INFINITY);
}
