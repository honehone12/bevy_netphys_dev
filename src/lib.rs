pub mod config;
pub mod level;
pub mod server_builder;
pub mod client_builder;
pub mod game_server;
pub mod game_client;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub type GameDefaultPhysics = ();

pub struct GameCommonPlugin;

impl Plugin for GameCommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            RapierPhysicsPlugin::<GameDefaultPhysics>::default()
        );
    }
}
