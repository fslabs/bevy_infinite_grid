use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
    render::view::Hdr,
};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FreeCameraPlugin, InfiniteGridPlugin))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(InfiniteGrid);

    commands.spawn((
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(0.0, 4.37, 14.77),
        FreeCamera::default(),
    ));

    commands.spawn((
        DirectionalLight { ..default() },
        Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial::default())),
        Transform::from_xyz(3.0, 4.0, 0.0)
            .with_rotation(Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()))
            .with_scale(Vec3::splat(1.5)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial::default())),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial::default())),
        Transform::from_xyz(0.0, -2.0, 0.0),
    ));
}
