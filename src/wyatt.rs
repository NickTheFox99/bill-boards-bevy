// <3 wyatt

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}

#[derive(Component)]
struct Wyatt;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("wyatt.glb"))),
        Wyatt,
        Transform::from_scale(Vec3::splat(1.0/7.0)).with_translation(Vec3::new(2.0,0.7,2.0))
    ));
}

fn update(
    player_pos: Query<&Transform, (With<crate::player::Player>, Without<Wyatt>)>,
    time: Res<Time>,
    mut wyatt_pos: Query<&mut Transform, With<Wyatt>>,
) {
    let mut wyatt = wyatt_pos.single_mut().unwrap();
    let player = player_pos.single().unwrap();

    let angle_to_player = Transform::from_translation(wyatt.translation)
        .looking_at(player.translation, Dir3::Y)
        .rotation.to_euler(EulerRot::YXZ).0;

    let current_quat = wyatt.rotation;
    let target_quat = Quat::from_euler(EulerRot::YXZ, angle_to_player, 0.0, 0.0);
    let new_rotation = current_quat.lerp(target_quat, 2.5 * time.delta_secs());

    wyatt.rotation = new_rotation;

    let dist = wyatt.translation.distance_squared(player.translation);

    let f = wyatt.forward();
    wyatt.translation += f * time.delta_secs() * (dist - 1.0) / 5.0;

}
