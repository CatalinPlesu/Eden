use bevy_rapier3d::{
    prelude::*,
};
use bevy::{
    prelude::*,
};
use bevy::prelude::shape;

pub fn spawn_gltf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let tree0 = ass.load("tree0.glb#Scene0");
    let tree1 = ass.load("tree1.glb#Scene0");
    let bush0 = ass.load("bush0.glb#Scene0");
    let bush1 = ass.load("bush1.glb#Scene0");

    // for i in -20..=20 {
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::rgb(1. + i as f32 / 10., 0.5, 1. + i as f32 / 20.),
    //             unlit: true,
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     })
    //     .insert(Transform::from_xyz(0., i as f32, 0.));
    // }

    commands.spawn_bundle((
        Transform::from_xyz(0.0, -10.0, 0.0),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(tree0);
    });

    commands.spawn_bundle((
        Transform::from_xyz(-6.0, -10.0, 0.0),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(tree1);
    });

    commands.spawn_bundle((
        Transform::from_xyz(0.0, -10.0, -5.0),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(bush0);
    });

    commands.spawn_bundle((
        Transform::from_xyz(3.0, -10.0, 0.0),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(bush1);
    });
}
