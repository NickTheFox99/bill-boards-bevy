use crate::MainCamera;
use bevy::prelude::*;

#[derive(Component)]
#[require(Mesh3d)]
pub struct Billboard;

fn update_billboards(
    mut boards: Query<&mut Transform, (With<Billboard>, Without<MainCamera>)>,
    cam: Query<&Transform, With<MainCamera>>,
) {
    let cam_transform = cam.single().unwrap();
    for mut transform in &mut boards {
        let target = transform.translation + Vec3::from(cam_transform.forward());
        transform.look_at(target, Vec3::Y);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_billboards);
}