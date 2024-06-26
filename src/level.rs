use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const FLOOR_SIZE: Vec3 = Vec3::new(60.0, 1.0, 60.0);
pub const FLOOR_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const FLOOR_POSITION: Vec3 = Vec3::new(0.0, -0.5, 0.0);
pub const LIGHT_POSITION: Vec3 = Vec3::new(0.0, 50.0, 0.0);
pub const LIGHT_ROTATION_X: f32 = -std::f32::consts::PI / 4.0;
pub const CAMERA_POSITION: Vec3 = Vec3::new(0.0, 70.0, 25.0);

pub fn client_setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(Cuboid::from_size(FLOOR_SIZE))),
        material: materials.add(FLOOR_COLOR),
        transform: Transform::from_translation(FLOOR_POSITION),
        ..default()
    })
    .insert(floor_collider());
}

pub fn server_setup_floor(mut commands: Commands) {
    commands.spawn(
        TransformBundle::from_transform(
            Transform::from_translation(FLOOR_POSITION)
        )
    )
    .insert(floor_collider());
}

pub fn floor_collider() -> Collider {
    let extents = FLOOR_SIZE * 0.5;
    Collider::cuboid(extents.x, extents.y, extents.z)
}

pub fn setup_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight{
            shadows_enabled: true,
            ..default()
        },
        transform: Transform{
            translation: LIGHT_POSITION,
            rotation: Quat::from_rotation_x(LIGHT_ROTATION_X),
            ..default()
        },
        ..default()
    });
}

pub fn setup_fixed_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle{
        transform: Transform::from_translation(CAMERA_POSITION)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
