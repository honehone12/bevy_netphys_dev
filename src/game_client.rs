use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::{
    *,
    network_rigidbody::*,
    level::*
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
            draw_net_rb_gizmos_system
        ))
        .add_systems(FixedUpdate, (
            update_net_rb_cache_system,
            apply_net_rb_interpolation_system
        ).chain(
        ).before(BEFORE_PHYSICS_SET));
    }
}

fn handle_player_spawned(
    mut commands: Commands,
    query: Query<
        (&NetworkRigidBody, Entity), 
        Added<NetworkRigidBody>
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>

) {
    for (net_rb, e) in query.iter() {
        let (trans, rot) = match net_rb {
            &NetworkRigidBody::ServerSimulation { translation, euler } => {
                (translation, Quat::from_euler(EulerRot::XYZ, 
                    euler.x, 
                    euler.y, 
                    euler.z
                ))
            }
            &NetworkRigidBody::ClientPrediction { translation, euler } => {
                (translation, Quat::from_euler(EulerRot::XYZ, 
                    euler.x, 
                    euler.y, 
                    euler.z
                ))
            }
        };

        commands.entity(e)
        .insert(
            PbrBundle{
                mesh: meshes.add(Mesh::from(Sphere::new(PLAYER_BALL_RADIUS))),
                material: materials.add(PLAYER_COLOR),
                transform: Transform{
                    translation: trans,
                    rotation: rot,
                    ..default()
                },
                ..default()
            }
        );

        match net_rb {
            NetworkRigidBody::ServerSimulation { .. } => {
                commands.entity(e)
                .insert(Cache::<NetworkRigidBody> {
                    latest: net_rb.clone(),
                    second: net_rb.clone(),
                    elapsed_time: -1.0
                })
                .insert(generate_kinematic_ball());
            },
            NetworkRigidBody::ClientPrediction { .. } => {
                commands.entity(e)
                .insert(generate_dynamic_ball());
            },
        }

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
        cache.second = if cache.elapsed_time < 0.0 {
            net_rb.clone()
        } else {
            cache.latest.clone()
        };
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
            NetworkRigidBody::ClientPrediction { .. } => panic!("client simulating RB"),
            NetworkRigidBody::ServerSimulation { translation, euler } => {
                (translation, Quat::from_euler(EulerRot::XYZ, 
                    euler.x, 
                    euler.y, 
                    euler.z
                ))
            } 
        };
        let (second_trans, second_rot) = match cache.second {
            NetworkRigidBody::ClientPrediction { .. } => panic!("client simulating RB"),
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
    
        transform.translation = translation;
        transform.rotation = rotation;

        cache.elapsed_time += fixed_time.delta_seconds();
    }
}

fn draw_net_rb_gizmos_system(
    query: Query<&NetworkRigidBody>,
    mut gizmos: Gizmos
) {
    for net_rb in query.iter() {
        let (trans, rot) = match net_rb {
            &NetworkRigidBody::ServerSimulation { .. } => return,
            &NetworkRigidBody::ClientPrediction { translation, euler } => {
                (translation, Quat::from_euler(EulerRot::XYZ, 
                    euler.x, 
                    euler.y, 
                    euler.z
                ))
            }
        };

        gizmos.sphere(
            trans, 
            rot, 
            PLAYER_BALL_RADIUS, 
            Color::GREEN
        );
    }
}