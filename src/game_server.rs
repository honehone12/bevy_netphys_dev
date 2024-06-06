use bevy::prelude::*;
use super::{
    *, 
    level::*
};

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameCommonPlugin)
        .add_systems(Startup, server_setup_floor);
    }
}