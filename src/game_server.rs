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
        .add_systems(PreUpdate, 
            handle_server_event
            .after(ServerSet::Receive)
        )
        .add_systems(Update, handle_fire)
        .add_systems(PostUpdate, 
            despawn_dropped
            .before(ServerSet::Send)
        )
        .add_systems(FixedUpdate, 
            set_network_rigidbody_system
            .after(AFTER_PHYSICS_SET)
        );
    }
}

fn handle_server_event(
    mut commands: Commands,
    mut events: EventReader<ServerEvent>
) {
    for e in events.read() {
        match e {
            ServerEvent::ClientConnected { client_id } => {
                commands.spawn((
                    Replicated,
                    NetworkId::new(*client_id)
                ));

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

fn handle_fire(
    mut commands: Commands,
    mut fire: EventReader<FromClient<NetworkFire>>
) {
    for FromClient { client_id, event: _ } in fire.read() {
        commands.spawn((
            Replicated,
            NetworkFireBall::new(*client_id),
            NetworkRigidBody::ClientPrediction {
                translation: BALL_SPAWN_POSITION,
                velocity: VELOCITY,
                euler: BALL_SPAWN_EULER,
                angular_velocity: ANGULAR_VELOCITY,
            },
            // NetworkRigidBody::ServerSimulation { 
            //     translation: BALL_SPAWN_POSITION, 
            //     euler: BALL_SPAWN_EULER 
            // },
            TransformBundle::from_transform(
                Transform{
                    translation: BALL_SPAWN_POSITION,
                    rotation: BALL_SPAWN_ROTATION,
                    ..default()
                }
            )
        ))
        .insert(generate_dynamic_ball(VELOCITY, ANGULAR_VELOCITY))
        // .insert(ExternalImpulse{
        //     impulse: IMPULSE,
        //     torque_impulse: TORQUE_IMPULSE,
        // });
        ;
    }
}

fn despawn_dropped(
    mut commands: Commands, 
    query: Query<(Entity, &Transform), With<RigidBody>>
) {
    for (e, transform) in query.iter() {
        if transform.translation.y < DROPPED_Y {
            commands.entity(e)
            .despawn();
        }
    }
}

fn set_network_rigidbody_system(
    mut query: Query<(
        Entity, 
        &Transform, 
        &mut NetworkRigidBody,
        &Velocity
    ), 
        With<RigidBody>
    >
) {
    for (e, transform, mut net_rb, vel) in query.iter_mut() {
        let trans = transform.translation;
        let e_rot = transform.rotation.to_euler(EulerRot::XYZ).into();
        
        match *net_rb {
            NetworkRigidBody::ServerSimulation { ref mut translation, ref mut euler } => {
                *translation = trans;
                *euler = e_rot;
            }
            NetworkRigidBody::ClientPrediction { 
                ref mut translation, 
                ref mut velocity,
                ref mut euler,
                ref mut angular_velocity, 
            } => {
                *translation = trans;
                *velocity = vel.linvel;
                *euler = e_rot;
                *angular_velocity = vel.angvel;
            }
        }
        
        info!(
            "rigidbody of entity: {e:?} translation: {} rotation: {} velocity: {} angular velocity: {}", 
            transform.translation,
            transform.rotation,
            vel.linvel,
            vel.angvel
        );
    }
}