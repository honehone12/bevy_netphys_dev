use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Serialize, Deserialize)]
pub enum NetworkRigidBody {
    ServerSimulation {
        translation: Vec3,
        euler: Vec3
    },
    ClientPrediction
}

pub struct NetworkRigidBodyPlugin;

impl Plugin for NetworkRigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<NetworkRigidBody>();
    }
}