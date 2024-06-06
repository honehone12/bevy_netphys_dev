pub mod config;
pub mod level;
pub mod server_builder;
pub mod client_builder;
pub mod game_server;
pub mod game_client;
pub mod network_rigidbody;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use config::*;
use network_rigidbody::*;

pub const PLAYER_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 25.0, 0.0);
pub const PLAYER_BALL_RADIUS: f32 = 1.0;
pub const PLAYER_BALL_RESTITUTION: f32 = 1.0;
pub const PLAYER_COLOR: Color = Color::RED;

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
        ));
    }
}
