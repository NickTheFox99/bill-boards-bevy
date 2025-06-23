use std::f32::consts::TAU;

use super::*;
use bevy::{image::*, prelude::*};

const CENTER_BILL_POS: Vec3 = Vec3::new(0.0, 1.0, 0.0);

#[derive(Resource)]
struct SinPhase(Timer);

#[derive(Component)]
struct Smile;

impl SinPhase {
    fn new() -> Self {
        SinPhase(Timer::from_seconds(TAU / 7.5, TimerMode::Repeating))
    }
}

impl Default for SinPhase {
    fn default() -> Self {
        Self::new()
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<SinPhase>();
    app.add_systems(Startup, setup);
    app.add_systems(PreUpdate, tick);
    app.add_systems(Update, bob);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FlatMaterial>>,
    assets: Res<AssetServer>,
) {
    let bill_smile: Handle<Image> = assets.load_with_settings("bill_smile.png", |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::nearest(),
            ..default()
        }
    });

    commands.spawn((
        billboard::Billboard::default(),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(0.5, 0.5)))),
        MeshMaterial3d(materials.add(FlatMaterial {
            texture: Some(bill_smile.clone()),
            color: Color::srgb_u8(0, 255, 0).into(),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_translation(CENTER_BILL_POS),
        Smile,
    ));
}

fn bob(mut faces: Query<&mut Transform, With<Smile>>, phase: ResMut<SinPhase>) {
    let mut face = faces.single_mut().unwrap();

    face.translation = CENTER_BILL_POS + Vec3::Y * (phase.0.elapsed_secs() * 7.5).sin() / 7.5;
}

fn tick(mut phase: ResMut<SinPhase>, time: Res<Time>) {
    phase.0.tick(time.delta());
}
