use bevy::{
    app::{App, CoreStage, Plugin},
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_rapier3d::{
    physics::wrapper::{ColliderMassPropsComponent, ColliderMaterialComponent},
    prelude::{
        ColliderBundle, ColliderMassProps, ColliderMaterial, ColliderPositionSync, ColliderShape,
        Isometry, NoUserData, Point, RapierPhysicsPlugin, Real, RigidBodyBundle, RigidBodyPosition,
        RigidBodyType,
    },
    render::RapierRenderPlugin,
};
use core::f32::consts::PI;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(LightPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(CubePlugin)
        .run();
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_camera);
    }
}

fn generate_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
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
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
            mass_properties: ColliderMassPropsComponent(ColliderMassProps::Density(1.)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Wireframe);
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
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_plane);
    }
}

fn generate_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_half = 50.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: plane_half * 2.0,
            })),
            material: materials.add(Color::rgba(0.2, 0.6, 0.2, 0.5).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(plane_half, 0., plane_half).into(),
            // mass_properties: ColliderMassPropsComponent(MassProperties::new(
            //     Vec3::new(0.0, -0.4, 0.0).into(),
            //     15.0,
            //     Vec3::new(100.0, 100.0, 100.0).into(),
            // )),
            // material: ColliderMaterialComponent(ColliderMaterial {
            //     friction: 1.0,
            //     restitution: 1_000_0000.0,
            //     ..Default::default()
            // }),
            ..Default::default()
        });

    // for x in -1..=1 {
    //     for z in -1..=1 {
    //         commands
    //             .spawn_bundle(PbrBundle {
    //                 mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
    //                 material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //                 transform: Transform::from_xyz(x as f32, 0., z as f32),
    //                 ..Default::default()
    //             })
    //             .insert_bundle(RigidBodyBundle {
    //                 body_type: RigidBodyType::Static.into(),
    //                 ..Default::default()
    //             })
    //             .insert_bundle(ColliderBundle {
    //                 shape: ColliderShape::cuboid(1., 1., 1.).into(),
    //                 // material: ColliderMaterialComponent ( ColliderMaterial{
    //                 //     friction: 1.0,
    //                 //     restitution: 1_000_0000.0,
    //                 //     ..Default::default()
    //                 // }),
    //                 ..Default::default()
    //             })
    //             .insert(Wireframe);
    //     }
    // }
}
