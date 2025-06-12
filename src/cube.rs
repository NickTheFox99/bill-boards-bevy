use crate::flat::{FlatMaterial, MaterialOverride};
use crate::{player, GameSettings};
use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use parry3d::math::{Isometry, Point};
use parry3d::na::Vector3;
use parry3d::query::{Ray, RayCast};
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
    mut materials: ResMut<Assets<FlatMaterial>>,
    mut cube_tex: ResMut<CubeTex>,
    assets: Res<AssetServer>,
) {
    cube_tex.0 = Some(assets.load_with_settings("cube_tex.png", |s: &mut _| {
        *s = ImageLoaderSettings {
            // sampler: ImageSampler::nearest(),
            ..default()
        }
    }));

    let cube_mesh = meshes.add(Cuboid::default());

    commands.spawn((
        Mesh3d::from(cube_mesh.clone()),
        MeshMaterial3d(materials.add(FlatMaterial {
            color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
            texture: Some(cube_tex.get_handle()),
        })),
        Transform::from_xyz(0.0, 0.25, 0.0).with_scale(Vec3::splat(0.5)),
        Cube,
    ));
}

fn cube_click_detect(
    mut s_mats: ResMut<Assets<StandardMaterial>>,
    mut f_mats: ResMut<Assets<FlatMaterial>>,
    g_set: Res<GameSettings>,
    players: Query<(&Transform, &ActionState<player::PlayerAction>), With<player::Player>>,
    cube_tex: Res<CubeTex>,
    cubes: Query<
        (
            &Transform,
            Option<&mut MeshMaterial3d<FlatMaterial>>,
            Option<&mut MeshMaterial3d<StandardMaterial>>,
        ),
        (With<Cube>, Without<MaterialOverride>),
    >,
) {
    let (p_trans, action) = players.single().unwrap();

    if !action.just_pressed(&player::PlayerAction::Click) {
        return;
    }

    let dir = p_trans.forward();
    let ray = Ray::new(
        Point::from(p_trans.translation.to_array()),
        Vector3::from(dir.to_array()),
    );

    for (c_trans, mut f_mat, mut s_mat) in cubes {
        let sq = parry3d::shape::Cuboid::new(Vector3::from(c_trans.scale.to_array()) * 0.5);

        let res = sq.cast_ray(
            &Isometry::new(
                Vector3::from(c_trans.translation.to_array()),
                Vector3::from(c_trans.rotation.to_scaled_axis().to_array()),
            ),
            &ray,
            50.0,
            true,
        );
        if let Some(_) = res {
            if g_set.contains(GameSettings::FLAT)
                && let Some(mut mat) = f_mat
            {
                mat.0 = f_mats.add(FlatMaterial {
                    color: Color::hsv(random::<f32>() * 360.0, 1.0, 1.0)
                        .with_alpha(1.0)
                        .into(),
                    texture: Some(cube_tex.get_handle()),
                });
            }

            if !g_set.contains(GameSettings::FLAT)
                && let Some(mut mat) = s_mat
            {
                mat.0 = s_mats.add(StandardMaterial {
                    base_color: Color::hsv(random::<f32>() * 360.0, 1.0, 1.0),
                    base_color_texture: Some(cube_tex.get_handle()),
                    ..default()
                });
            }
        }
    }
}
