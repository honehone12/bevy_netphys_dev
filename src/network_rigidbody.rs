use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Serialize, Deserialize, Clone)]
pub enum NetworkRigidBody {
    ServerSimulation {
        translation: Vec3,
        euler: Vec3
    },
    ClientPrediction
}

impl NetworkRigidBody {
    #[inline]
    pub fn default_server_simulation() -> Self {
        Self::ServerSimulation { 
            translation: default(), 
            euler: default() 
        }
    }
}

pub struct NetworkRigidBodyPlugin;

impl Plugin for NetworkRigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<NetworkRigidBody>();
    }
}