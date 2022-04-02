use bevy::{app::Plugin, prelude::*};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 250_000.0,
            range: 2_000.0,
            shadows_enabled: true,
            shadow_depth_bias: 1.,
            shadow_normal_bias: 1.,
            ..Default::default()
        },
        transform: Transform::from_xyz(50., 100.0, 50.),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 50_000.0,
            range: 60.0,
            shadows_enabled: true,
            shadow_depth_bias: 1.,
            shadow_normal_bias: 1.,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 20.0, 0.),
        ..Default::default()
    });
}
