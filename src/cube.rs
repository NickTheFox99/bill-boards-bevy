use crate::player;
use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use parry2d::math::Point;
use parry2d::na::{Isometry2, Vector2};
use parry2d::query::{Ray, RayCast};
use rand::random;

#[derive(Component)]
#[require(Mesh3d)]
pub struct Cube;

#[derive(Resource)]
struct CubeTex(Option<Handle<Image>>);

impl CubeTex {
    /// Return a cloned handle to the Render Texture image.
    pub fn get_handle(&self) -> Handle<Image> {
        self.0.clone().unwrap().clone()
    }
}

impl Default for CubeTex {
    fn default() -> Self {
        CubeTex(None)
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<CubeTex>();
    app.add_systems(Startup, setup);
    app.add_systems(Update, cube_click_detect);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cube_tex: ResMut<CubeTex>,
    assets: Res<AssetServer>,
) {
    cube_tex.0 = Some(assets.load_with_settings("cube_tex.png", |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::nearest(),
            ..default()
        }
    }));

    let cube_mesh = meshes.add(Cuboid::default());

    commands.spawn((
        Mesh3d::from(cube_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(255, 0, 0),
            base_color_texture: Some(cube_tex.get_handle()),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.25, 0.0).with_scale(Vec3::splat(0.5)),
        Cube,
    ));
}

fn cube_click_detect(
    players: Query<(&Transform, &ActionState<player::PlayerAction>), With<player::Player>>,
    cube_tex: Res<CubeTex>,
    cubes: Query<(&Transform, &mut MeshMaterial3d<StandardMaterial>), With<Cube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (player, action) = players.single().unwrap();

    if !action.just_pressed(&player::PlayerAction::CubeClick) {
        return;
    }

    let dir = player.forward();
    let ray = Ray::new(Point::new(player.translation.x, player.translation.z), Vector2::new(dir.x, dir.z));

    for (trans, mut mat) in cubes {
        let sq = parry2d::shape::Cuboid::new(Vector2::new(trans.scale.x, trans.scale.z) * 0.5);
        let res = sq.cast_ray(&Isometry2::new(Vector2::new(trans.translation.x, trans.translation.z), 0.0), &ray, 50.0, true);
        if let Some(_) = res {
            mat.0 = materials.add(StandardMaterial {
                base_color: Color::hsv(random::<f32>() * 360.0, 1.0, 1.0),
                base_color_texture: Some(cube_tex.get_handle()),
                unlit: true,
                ..default()
            })
        }
    }
}
