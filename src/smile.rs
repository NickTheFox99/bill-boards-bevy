use super::*;
use crate::sinphase::SinPhase;
use bevy::{image::*, prelude::*};

const CENTER_BILL_POS: Vec3 = Vec3::new(0.0, 1.0, 0.0);

#[derive(Component)]
struct Smile;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
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
        SinPhase::default(),
    ));
}

fn bob(mut faces: Query<(&mut Transform, &SinPhase), With<Smile>>) {
    let (mut face, phase) = faces.single_mut().unwrap();
    face.translation = CENTER_BILL_POS + Vec3::Y * phase.get_phase();
}
