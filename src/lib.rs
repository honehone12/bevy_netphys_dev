pub mod config;
pub mod level;
pub mod server_builder;
pub mod client_builder;
pub mod game_server;
pub mod game_client;
pub mod network_rigidbody;

use bevy_replicon::prelude::AppRuleExt;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_rapier3d::prelude::*;
use config::*;
use network_rigidbody::*;

pub const BALL_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 15.0, 0.0);
pub const BALL_SPAWN_EULER: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const BALL_SPAWN_ROTATION: Quat = Quat::IDENTITY;
pub const BALL_RADIUS: f32 = 1.0;
pub const BALL_RESTITUTION: f32 = 0.8;
pub const BALL_COLOR: Color = Color::RED;

pub const IMPULSE: Vec3 = Vec3::new(0.0, 100.0, 0.0);
pub const TORQUE_IMPULSE: Vec3 = Vec3::new(5.0, 5.0, 0.0);

pub const VELOCITY: Vec3 = Vec3::new(0.0, 30.0, 0.0);
pub const ANGULAR_VELOCITY: Vec3 = Vec3::new(5.0, 5.0, 0.0);

pub const DROPPED_Y: f32 = -15.0;

pub const BEFORE_PHYSICS_SET: PhysicsSet = PhysicsSet::SyncBackend;
pub const AFTER_PHYSICS_SET: PhysicsSet = PhysicsSet::Writeback;

pub const FIRE_KEY: KeyCode = KeyCode::Space;

#[derive(Component, Serialize, Deserialize)]
pub struct NetworkId(ClientId);

impl NetworkId {
    #[inline]
    pub fn new(client_id: ClientId) -> Self {
        Self(client_id)
    }

    #[inline]
    pub fn client_id(&self) -> ClientId {
        self.0
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct NetworkFireBall(ClientId);

impl NetworkFireBall {
    #[inline]
    pub fn new(caster: ClientId) -> Self {
        Self(caster)
    }

    #[inline]
    pub fn caster(&self) -> ClientId {
        self.0
    }
}

#[derive(Event, Serialize, Deserialize)]
pub struct NetworkFire;

#[derive(Component)]
pub struct Cache<C: Component> {
    pub latest: C,
    pub second: C,
    pub elapsed_time: f32
}

pub struct GameCommonPlugin;

impl Plugin for GameCommonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RapierConfiguration>();

        let mut physics_config = app.world.resource_mut
        ::<RapierConfiguration>();
        physics_config.timestep_mode = TimestepMode::Fixed {
            dt: PHYSICS_FIXED_TICK_DELTA, 
            substeps: 1 
        };

        app.add_plugins((
            RapierPhysicsPlugin::<()>::default()
            .in_fixed_schedule(),
            NetworkRigidBodyPlugin
        ))
        .replicate::<NetworkId>()
        .replicate::<NetworkFireBall>()
        .add_client_event::<NetworkFire>(ChannelKind::Ordered);
    }
}

pub(crate) fn generate_p_kinematic_ball() -> impl Bundle {
    (
        RigidBody::KinematicPositionBased,
        Collider::ball(BALL_RADIUS)
    )
}

pub(crate) fn generate_v_kinematic_ball(velocity: Vec3, angular_velocity: Vec3) 
-> impl Bundle {
    (
        RigidBody::KinematicVelocityBased,
        Velocity{
            linvel: velocity,
            angvel: angular_velocity
        },
        Collider::ball(BALL_RADIUS)
    )
}

pub(crate) fn generate_dynamic_ball(velocity: Vec3, angular_velocity: Vec3) 
-> impl Bundle {
    (
        RigidBody::Dynamic,
        Velocity{
            linvel: velocity,
            angvel: angular_velocity
        },
        Collider::ball(BALL_RADIUS),
        Ccd::enabled(),
        Restitution::coefficient(BALL_RESTITUTION),
    )
}

pub fn euler_to_quat(euler: Vec3) -> Quat {
    Quat::from_euler(EulerRot::XYZ, euler.x, euler.y, euler.z)
}

pub fn quat_to_euler(quat: Quat) -> Vec3 {
    quat.to_euler(EulerRot::XYZ).into()
}
