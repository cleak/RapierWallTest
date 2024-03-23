use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const WIN_SZ: Vec2 = Vec2::new(1280., 720.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rapier KCC Test".into(),
                resolution: WIN_SZ.into(),
                ..default()
            }),
            ..default()
        }))
        .register_type::<FpsController>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            brightness: 5.0,
            ..default()
        })
        .add_systems(Startup, (setup_scene_walls, setup_player))
        .add_systems(Update, fps_move)
        .run();
}

/// Sets up a basic scene with a ground plane and 3 walls.
pub fn setup_scene_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat_blue = materials.add(
        StandardMaterial {
            base_color_texture: Some(assets.load("grid_blue.png")),
            ..default()
        }
    );

    let mat_gray = materials.add(
        StandardMaterial {
            base_color_texture: Some(assets.load("grid_gray.png")),
            ..default()
        }
    );

    let mut make_cube = |pos: Vec3, size: Vec3, mat: &Handle<StandardMaterial>| {
        return commands.spawn((PbrBundle {
                mesh: meshes.add(Cuboid::from_size(size)),
                material: mat.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                ..default()
            },
            Collider::cuboid(size.x/2.0, size.y/2.0, size.z/2.0),
        )).id();
    };

    // Ground
    make_cube(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(40., 0.01, 40.),
        &mat_blue
    );

    // Walls
    make_cube(
        Vec3::new(0.0, 4.0, -15.0),
        Vec3::new(12., 8.0, 0.25),
        &mat_gray
    );

    make_cube(
        Vec3::new(-6.0, 4.0, -9.0),
        Vec3::new(0.25, 8.0, 12.),
        &mat_gray
    );

    make_cube(
        Vec3::new(6.0, 4.0, -9.0),
        Vec3::new(0.25, 8.0, 12.),
        &mat_gray
    );

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI * -0.15,
            PI * -0.15,
        )),
        ..default()
    });
}

/// Creates a simple camera loking at the scene.
fn setup_player(
    mut cmds: Commands,
) {
    let cam = cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 1.65, 0.),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        Name::new("Main Camera"),
    ))
    .id();

    cmds.spawn(
        (
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            },
            FpsController {
                speed_turn: 3.0,
                speed_move: 4.0,
            },
            Collider::capsule(Vec3::ZERO, Vec3::new(0., 1.3, 0.), 0.33),
            KinematicCharacterController {
                // All of these values produce the same getting stuck on walls problem.
                // offset: CharacterLength::Relative(0.01),
                // offset: CharacterLength::Relative(0.10),
                // offset: CharacterLength::Relative(1.00),
                .. default()
            },
            Name::new("Player"),
        ))
        .add_child(cam);
}

#[derive(Reflect, Component, Default)]
struct FpsController {
    speed_move: f32,
    speed_turn: f32,
}

/// Player movement
fn fps_move(
    kbd: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<(&mut KinematicCharacterController, &FpsController, &mut Transform)>,
) {
    let mut delta_move = 0.;
    let mut delta_turn = 0.;

    if kbd.pressed(KeyCode::KeyW) || kbd.pressed(KeyCode::ArrowUp) {
        delta_move = 1.;
    }

    if kbd.pressed(KeyCode::KeyS) || kbd.pressed(KeyCode::ArrowDown) {
        delta_move = -1.;
    }

    if kbd.pressed(KeyCode::KeyD) || kbd.pressed(KeyCode::ArrowRight) {
        delta_turn = -1.;
    }

    if kbd.pressed(KeyCode::KeyA) || kbd.pressed(KeyCode::ArrowLeft) {
        delta_turn = 1.;
    }

    let t = time.delta_seconds().clamp(0.001, 0.100);

    for (mut kcc, fps_controller, mut transform) in &mut players {
        let delta_turn = delta_turn * fps_controller.speed_turn * t;
        if delta_turn.abs() > 0. {
            transform.rotate(Quat::from_axis_angle(Vec3::Y, delta_turn));
        }
        
        let delta_world = transform.forward() * delta_move * fps_controller.speed_move * t;
        if delta_world.dot(delta_world) > 0. {
            kcc.translation = Some(delta_world);
        }
    }
}
