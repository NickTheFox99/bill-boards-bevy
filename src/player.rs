use crate::display::RenderTex;
use crate::MainCamera;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;
use leafwing_input_manager::prelude::*;
use parry2d::math::Translation;
use parry2d::na::Vector2;
use parry2d::query;
use parry2d::shape::{Ball, Cuboid};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

const PLAYER_SPEED: f32 = 5.4;

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

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.add_systems(Startup, setup);
    app.add_systems(Update, ((movement_input, physics).chain(), mouselook));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    Look,
    Click,
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

        // Default kbm input bindings
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Look, MouseMove::default());
        input_map.insert(Self::Click, MouseButton::Left);

        input_map
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Player {
    yaw: f32,
    pitch: f32,
    move_input: Option<Vec2>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            yaw: 0.0,
            pitch: 0.0,
            move_input: None,
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Moving;

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

    player.move_input = Some(direction.xz() * time.delta_secs() * PLAYER_SPEED);
}

fn physics(
    mut player_query: Query<(&mut Transform, &Player), Without<crate::cube::Cube>>,
    cubes: Query<&Transform, With<crate::cube::Cube>>,
) {
    let (mut trans, player) = player_query.single_mut().unwrap();

    if let Some(slide) = player.move_input {
        let p_shape = Ball::new(0.25);
        trans.translation += Vec3::new(slide.x, 0.0, slide.y);

        // Push-Out
        for cube in cubes {
            let boid = Cuboid::new(Vector2::new(cube.scale.x, cube.scale.z) * 0.5);

            if let Ok(Some(res)) = query::contact(
                &Translation::new(trans.translation.x, trans.translation.z).into(),
                &p_shape,
                &Translation::new(cube.translation.x, cube.translation.z).into(),
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
