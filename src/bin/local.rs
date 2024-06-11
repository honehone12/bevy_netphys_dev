use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_netphys_dev::{
    *,
    level::*,
    config::*
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
    .init_resource::<RapierConfiguration>();

    let mut physics_config = app.world.resource_mut::<RapierConfiguration>();
    physics_config.timestep_mode = TimestepMode::Fixed {
        dt: PHYSICS_FIXED_TICK_DELTA, 
        substeps: 1 
    };

    app.add_plugins((
        RapierPhysicsPlugin::<()>::default()
        .in_fixed_schedule(),
        RapierDebugRenderPlugin::default()
    ))
    .add_systems(Startup, (
        setup_light,
        setup_fixed_camera,
        client_setup_floor,
        setup_ball
    ).chain())
    .run();
}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Mesh::from(Sphere::new(BALL_RADIUS))),
            material: materials.add(BALL_COLOR),
            transform: Transform::from_translation(BALL_SPAWN_POSITION),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Restitution::coefficient(BALL_RESTITUTION),
        ExternalImpulse{
            impulse: Vec3::ZERO,
            torque_impulse: TORQUE_IMPULSE,
        }
    ));
}