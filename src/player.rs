use crate::MainCamera;
use crate::display::RenderTex;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;
use leafwing_input_manager::prelude::*;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        ((movement_input, physics, stepping).chain(), mouselook),
    );
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    Look,
    Click,
    Sprint,
}

impl PlayerAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT.with_circle_deadzone(0.05));
        input_map.insert_dual_axis(
            Self::Look,
            GamepadStick::RIGHT
                .with_circle_deadzone(0.05)
                .inverted_y()
                .sensitivity_x(20.0)
                .sensitivity_y(16.0),
        );
        input_map.insert(Self::Click, GamepadButton::RightTrigger2);
        input_map.insert(Self::Sprint, GamepadButton::LeftTrigger2);

        // Default kbm input bindings
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Look, MouseMove::default());
        input_map.insert(Self::Click, MouseButton::Left);
        input_map.insert(Self::Sprint, KeyCode::Space);

        input_map
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Player {
    move_speed: f32,
    sprint_speed: f32,
    yaw: f32,
    pitch: f32,
    move_input: Option<Vec2>,
    height: f32,
    fat: f32,
    step_dist: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            move_speed: 5.4,
            sprint_speed: 8.1,
            yaw: 0.0,
            pitch: 0.0,
            move_input: None,
            height: 1.0,
            fat: f32::MIN_POSITIVE,
            step_dist: 0.05,
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Moving;

fn setup(mut commands: Commands, render_tex: Res<RenderTex>) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            // clear_color: ClearColorConfig::None,
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(245, 245, 245)),
            target: RenderTarget::Image(render_tex.get_handle().into()),
            ..default()
        },
        Projection::from(PerspectiveProjection {
            fov: FRAC_PI_4,
            aspect_ratio: 4.0 / 3.0,
            near: 0.0001,
            ..default()
        }),
        Transform::from_xyz(5.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
        MainCamera,
        Player::default(),
        PlayerAction::default_input_map(),
        RenderLayers::layer(0),
    ));
}

fn mouselook(mut query: Query<(&mut Transform, &mut Player, &ActionState<PlayerAction>)>) {
    let (mut transform, mut player_camera, action) = query.single_mut().unwrap();

    let data = action.axis_pair(&PlayerAction::Look) * 0.00625 / 6.0;
    player_camera.yaw -= data.x;
    player_camera.pitch -= data.y;
    player_camera.pitch = player_camera.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

    transform.rotation = Quat::from_axis_angle(Vec3::Y, player_camera.yaw)
        * Quat::from_axis_angle(Vec3::X, player_camera.pitch);
}

fn movement_input(
    mut player_query: Query<(&mut Player, &ActionState<PlayerAction>)>,
    time: Res<Time>,
) {
    let (mut player, action) = player_query.single_mut().unwrap();

    let data = action.axis_pair(&PlayerAction::Move);
    let mut direction = Vec3::new(data.x, 0.0, -data.y);

    if direction.length_squared() <= 0.0 {
        player.move_input = None;
        ()
    }

    if direction.length_squared() > 1.0 {
        direction = direction.normalize();
    }

    let rotation = Quat::from_rotation_y(player.yaw);
    direction = rotation.mul_vec3(direction);

    let move_factor;

    if action.pressed(&PlayerAction::Sprint) {
        move_factor = player.sprint_speed;
    } else {
        move_factor = player.move_speed;
    }

    player.move_input = Some(direction.xz() * time.delta_secs() * move_factor);
}

fn physics(
    mut player_query: Query<(&mut Transform, &Player), Without<crate::cube::Cube>>,
    cubes: Query<&Transform, With<crate::cube::Cube>>,
) {
    let (mut trans, player) = player_query.single_mut().unwrap();

    if let Some(slide) = player.move_input {
        let p_shape = parry2d::shape::Ball::new(player.fat);
        trans.translation += Vec3::new(slide.x, 0.0, slide.y);

        // Push-Out
        for cube in cubes {
            let boid = parry2d::shape::Cuboid::new(
                parry2d::na::Vector2::new(cube.scale.x, cube.scale.z) * 0.5,
            );

            if let Ok(Some(res)) = parry2d::query::contact(
                &parry2d::math::Translation::new(trans.translation.x, trans.translation.z).into(),
                &p_shape,
                &parry2d::math::Translation::new(cube.translation.x, cube.translation.z).into(),
                &boid,
                0.0,
            ) {
                if res.dist < 0.0 {
                    let push_out = res.normal1.into_inner() * res.dist;
                    trans.translation.x += push_out.x;
                    trans.translation.z += push_out.y;
                }
            };
        }
    };
}

fn stepping(
    mut player_query: Query<(&mut Transform, &Player), Without<crate::cube::Cube>>,
    cubes: Query<&Transform, With<crate::cube::Cube>>,
) {
    let (mut trans, player) = player_query.single_mut().unwrap();
    let mut highest_point: f32 = f32::NEG_INFINITY;
    let p_shape = parry2d::shape::Ball::new(player.fat + player.step_dist);
    for cube in cubes.iter() {
        let boid = parry2d::shape::Cuboid::new(
            parry2d::na::Vector2::new(cube.scale.x, cube.scale.z) * 0.5,
        );

        if !parry2d::query::intersection_test(
            &parry2d::math::Translation::new(trans.translation.x, trans.translation.z).into(),
            &p_shape,
            &parry2d::math::Translation::new(cube.translation.x, cube.translation.z).into(),
            &boid,
        )
        .unwrap()
        {
            continue;
        }

        let height = cube.translation.y + cube.scale.y * 0.5;
        if height > highest_point {
            highest_point = height;
        }
    }

    if highest_point > f32::NEG_INFINITY {
        trans.translation.y = highest_point + player.height;
    } else {
        trans.translation.y = player.height;
    }
}
