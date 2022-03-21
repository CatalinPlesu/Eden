use bevy::{
    app::{App, CoreStage, Plugin},
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_rapier3d::{
    physics::wrapper::{
        ColliderMassPropsComponent, ColliderMaterialComponent, RigidBodyMassPropsComponent,
    },
    prelude::{
        CoefficientCombineRule, ColliderBundle, ColliderMassProps, ColliderMaterial,
        ColliderPositionSync, ColliderShape, Isometry, NoUserData, Point, RapierPhysicsPlugin,
        Real, RigidBodyBundle, RigidBodyForces, RigidBodyMassProps, RigidBodyPosition,
        RigidBodyPositionSync, RigidBodyType, RigidBodyMassPropsFlags,
    },
    render::RapierRenderPlugin,
};

use bevy_flycam::PlayerPlugin as FlyCam;

use core::f32::consts::PI;

mod input;
mod world;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(WireframePlugin)
        .add_plugin(RapierRenderPlugin)
        // .add_plugin(input::PlayerPlugin)
        .add_plugin(FlyCam)
        .add_plugin(LightPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(CubePlugin)
        .add_plugin(BallsPlugin)
        .run();
}

pub struct CubePlugin;
impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_cube);
    }
}

fn generate_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.9, 0.5, 0.5).into()),
            ..Default::default()
        })
        .insert(Wireframe)
        .insert_bundle(RigidBodyBundle {
            position: RigidBodyPosition {
                position: Isometry::new(
                    Vec3::new(0.0, 3.0, 0.0).into(),
                    Vec3::new(PI / 4.0, PI / 4.0, PI / 4.0).into(),
                ),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
            ..Default::default()
        })
        .insert(Transform::default())
        .insert(ColliderPositionSync::Discrete);
}

pub struct BallsPlugin;
impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_balls);
    }
}

fn generate_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.25;

    for y in (-100..=100).step_by(10) {
        for x in (-100..=100).step_by(10) {
            let x01 = (x + 5) as f32 / 10.0;
            let y01 = (y + 2) as f32 / 4.0;
            // sphere
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: radius,
                        subdivisions: 32,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::hex("ff9191").unwrap(),
                        // vary key PBR parameters on a grid of spheres to show the effect
                        metallic: y01,
                        perceptual_roughness: x01,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert_bundle(RigidBodyBundle {
                    position: RigidBodyPosition {
                        position: Isometry::new(
                            Vec3::new(x as f32, 5.0, y as f32).into(),
                            Vec3::new(0., 0., 0.).into(),
                        ),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::ball(radius).into(),
                    ..Default::default()
                })
                .insert(Transform::default())
                .insert(ColliderPositionSync::Discrete);
        }
    }
}

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_light);
    }
}

fn generate_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 2900_000.0,
            range: 2000.0,
            shadows_enabled: true,
            shadow_depth_bias: 1.,
            shadow_normal_bias: 1.,
            ..Default::default()
        },
        transform: Transform::from_xyz(-50., 200.0, -50.),
        ..Default::default()
    });
}
