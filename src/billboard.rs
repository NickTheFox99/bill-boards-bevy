use crate::{player, MainCamera};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use parry3d::math::{Isometry, Point};
use parry3d::na::{Point3, Vector3};
use parry3d::query::{Ray, RayCast};
use std::f32::consts::{ TAU};

#[derive(Component)]
#[require(Mesh3d)]
pub struct Billboard {
    spin: f32,
}

impl Default for Billboard {
    fn default() -> Self {
        Billboard { spin: 0.0 }
    }
}

fn face_billboards(
    mut boards: Query<&mut Transform, (With<Billboard>, Without<MainCamera>)>,
    cam: Query<&Transform, With<MainCamera>>,
) {
    let cam_transform = cam.single().unwrap();
    for mut transform in &mut boards {
        transform.rotation = Quat::IDENTITY;
        let target = transform.translation + Vec3::from(cam_transform.forward());
        transform.look_at(target, Vec3::Y);
    }
}

fn billboard_interaction(
    boards: Query<(&mut Billboard, &Transform)>,
    players: Query<
        (
            &Transform,
            &ActionState<player::PlayerAction>,
        ),
        (With<player::Player>, Without<Billboard>),
    >,
    time: Res<Time>,
) {
    let (p_trans, action) = players.single().unwrap();

    if !action.pressed(&player::PlayerAction::Click) {
        return;
    }

    let dir = p_trans.forward();
    let ray = Ray::new(
        Point::new(
            p_trans.translation.x,
            p_trans.translation.y,
            p_trans.translation.z,
        ),
        Vector3::new(dir.x, dir.y, dir.z),
    );

    let points = vec![
        Point3::new(0.5, 0.5, 0.0),
        Point3::new(0.5, -0.5, 0.0),
        Point3::new(-0.5, -0.5, 0.0),
        Point3::new(-0.5, 0.5, 0.0),
    ];

    let indices = vec![[0u32, 1, 3], [2, 3, 1]];

    let quad = parry3d::shape::TriMesh::new(points, indices).unwrap();

    for (mut bill, b_trans) in boards {
        let b_mesh = quad.clone();
        let aa = b_trans.rotation.to_axis_angle();

        let res = b_mesh.cast_ray(
            &Isometry::new(
                Vector3::from(b_trans.translation.to_array()),
                Vector3::from((aa.0 * aa.1).to_array()),
            ),
            &ray,
            50.0,
            true,
        );

        if let Some(_) = res {
            bill.spin += TAU * time.delta_secs();
        }
    }
}

fn rot_boards(boards: Query<(&mut Transform, &Billboard)>) {
    for (mut trans, bill) in boards {
        trans.rotate_local_z(bill.spin);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, face_billboards);
    app.add_systems(Update, (billboard_interaction, rot_boards).chain());
}
