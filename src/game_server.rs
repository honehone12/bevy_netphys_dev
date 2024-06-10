use bevy::prelude::*;
use bevy_replicon::prelude::*;
use super::{
    *, 
    level::*,
    network_rigidbody::*
};

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameCommonPlugin)
        .add_systems(Startup, server_setup_floor)
        .add_systems(Update, handle_server_event)
        .add_systems(FixedUpdate, 
            set_network_rigidbody_system
            .after(AFTER_PHYSICS_SET)
        );
    }
}

fn handle_server_event(
    mut commnads: Commands,
    mut events: EventReader<ServerEvent>
) {
    for e in events.read() {
        match e {
            ServerEvent::ClientConnected { client_id } => {
                commnads.spawn((
                    Replicated,
                    TransformBundle::from_transform(
                        Transform::from_translation(PLAYER_SPAWN_POSITION)
                    ),
                    // NetworkRigidBody::ServerSimulation{ 
                    //     translation: PLAYER_SPAWN_POSITION, 
                    //     euler: default() 
                    // },
                    NetworkRigidBody::ClientPrediction{
                        translation: PLAYER_SPAWN_POSITION, 
                        euler: default() 
                    }
                ))
                .insert(generate_dynamic_ball());

                info!("client: {client_id:?} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!(
                    "client: {:?} disconnected with reason: {}",
                    client_id, reason
                );
            }
        }
    }
}

fn set_network_rigidbody_system(
    mut query: Query<
        (Entity, &Transform, &mut NetworkRigidBody), 
        With<RigidBody>
    >
) {
    for (e, transform, mut net_rb) in query.iter_mut() {
        let trans = transform.translation;
        let e_rot = transform.rotation.to_euler(EulerRot::XYZ).into();
        
        match *net_rb {
            NetworkRigidBody::ServerSimulation { ref mut translation, ref mut euler } => {
                *translation = trans;
                *euler = e_rot;
            }
            NetworkRigidBody::ClientPrediction { ref mut translation, ref mut euler } => {
                *translation = trans;
                *euler = e_rot;
            }
        }
        
        info!(
            "rigidbody of entity: {e:?} translation: {} rotation: {}", 
            transform.translation,
            transform.rotation
        );
    }
}