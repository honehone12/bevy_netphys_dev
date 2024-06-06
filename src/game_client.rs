use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::{
    *,
    network_rigidbody::*,
    level::*,
    config::DEV_NETWORK_TICK_DELTA
};

pub struct GameClientPlugin;

impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameCommonPlugin,
            RapierDebugRenderPlugin::default()
        ))
        .add_systems(Startup, (
            setup_light,
            setup_fixed_camera,
            client_setup_floor
        ))
        .add_systems(Update, (
            handle_player_spawned,
            update_net_rb_cache_system,
            apply_net_rb_interpolation_system
        ).chain());
    }
}

fn handle_player_spawned(
    mut commands: Commands,
    query: Query<Entity, Added<NetworkRigidBody>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>

) {
    for e in query.iter() {
        commands.entity(e)
        .insert((
            PbrBundle{
                mesh: meshes.add(Mesh::from(Sphere::new(PLAYER_BALL_RADIUS))),
                material: materials.add(PLAYER_COLOR),
                transform: default(),
                ..default()
            },
            Cache::<NetworkRigidBody> {
                latest: NetworkRigidBody::default_server_simulation(),
                second: NetworkRigidBody::default_server_simulation(),
                elapsed_time: default()
            },
            RigidBody::KinematicPositionBased,
            Collider::ball(PLAYER_BALL_RADIUS)
        ));

        info!("player entity: {e:?} spawned");
    }
}

fn update_net_rb_cache_system(
    mut query: Query<
        (&NetworkRigidBody, &mut Cache<NetworkRigidBody>),
        Changed<NetworkRigidBody>
    >
) {
    for (net_rb, mut cache) in query.iter_mut() {
        cache.second = cache.latest.clone();
        cache.latest = net_rb.clone();
        cache.elapsed_time = 0.0;
    }
}

fn apply_net_rb_interpolation_system(
    mut query: Query<(
        &mut Cache<NetworkRigidBody>, 
        &mut Transform
    )>,
    fixed_time: Res<Time<Fixed>>
) {
    for (mut cache, mut transform) in query.iter_mut() {
        let (latest_trans, latest_rot) = match cache.latest {
            NetworkRigidBody::ClientPrediction => unimplemented!(),
            NetworkRigidBody::ServerSimulation { translation, euler } => {
                (translation, Quat::from_euler(EulerRot::XYZ, 
                    euler.x, 
                    euler.y, 
                    euler.z
                ))
            } 
        };
        let (second_trans, second_rot) = match cache.second {
            NetworkRigidBody::ClientPrediction => unimplemented!(),
            NetworkRigidBody::ServerSimulation { translation, euler } => {
                (translation, Quat::from_euler(EulerRot::XYZ, 
                    euler.x, 
                    euler.y, 
                    euler.z
                ))
            } 
        };

        let per = (cache.elapsed_time / DEV_NETWORK_TICK_DELTA)
        .clamp(0.0, 1.0);
        let translation = second_trans.lerp(latest_trans, per);
        let rotation = second_rot.slerp(latest_rot, per);
        cache.elapsed_time += fixed_time.delta_seconds();

        transform.translation = translation;
        transform.rotation = rotation;
    }
}
