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
            apply_network_rigidbody_system
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
        .insert(PbrBundle{
            mesh: meshes.add(Mesh::from(Sphere::new(PLAYER_BALL_RADIUS))),
            material: materials.add(PLAYER_COLOR),
            transform: default(),
            ..default()
        });

        info!("player entity: {e:?} spawned");
    }
}

fn apply_network_rigidbody_system(
    mut query: Query<(&NetworkRigidBody, &mut Transform)>
) {
    for (net_rb, mut transform) in query.iter_mut() {
        let (translation, rotation) = match net_rb {
            &NetworkRigidBody::ClientPrediction => unimplemented!(),
            &NetworkRigidBody::ServerSimulation { translation, euler } => {
                (translation, Quat::from_euler(EulerRot::XYZ, 
                    euler.x, 
                    euler.y, 
                    euler.z
                ))
            } 
        };
        
        transform.translation = translation;
        transform.rotation = rotation;
    }
}
