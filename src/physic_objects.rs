use bevy::prelude::{Commands, Component};
use parry2d::na::Vector2;
use parry2d::shape::{Cuboid, Shape};

pub(crate) fn setup(
    mut commands: Commands
) {
    commands.spawn((
        Floor {
            top: 0.0,
            shape: Cuboid::new(Vector2::new(5.0, 5.0))
        }
    ));
}

#[derive(Component)]
struct Floor<T: Shape> {
    top: f32,
    shape: T,
}

#[derive(Component)]
struct Wall<T: Shape> {
    shape: T,
}