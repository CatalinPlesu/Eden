use bevy::{
    app::{App, CoreStage, Plugin},
    diagnostic::FrameTimeDiagnosticsPlugin,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_rapier3d::{
    na::Point3,
    physics::wrapper::{
        ColliderMassPropsComponent, ColliderMaterialComponent, RigidBodyMassPropsComponent,
    },
    prelude::{
        CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMassProps, ColliderMaterial,
        ColliderPositionSync, ColliderShape, InteractionGroups, Isometry, NoUserData, Point,
        RapierPhysicsPlugin, Real, RigidBodyBundle, RigidBodyForces, RigidBodyMassProps,
        RigidBodyMassPropsFlags, RigidBodyPosition, RigidBodyPositionSync, RigidBodyType,
    },
    render::RapierRenderPlugin,
};

use bevy_flycam::PlayerPlugin as FlyCam;
use core::f32::consts::PI;
use rand::prelude::*;

use player::*;
mod player;
mod ui;
mod world;

pub struct WorldSettings {
    size: f32,
    plants: u32,
    plant_dominance_offset: f32,
    plants_colider: bool,
    fruits: i32,
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            size: 200.,
            plants: 300,
            plant_dominance_offset: 0.,
            plants_colider: true,
            fruits: 100,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Eden"),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WorldSettings::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(WireframePlugin)
        .add_plugin(RapierRenderPlugin)
        // .add_plugin(player::PlayerPlugin)
        // .add_plugin(FlyCam)
        .add_plugin(PlayerPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(CubePlugin)
        .add_plugin(FruitsPlugin)
        .add_plugin(ui::UserInterfacePlugin)
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

pub struct FruitsPlugin;
impl Plugin for FruitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_balls);
    }
}

fn generate_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>,
    ass: Res<AssetServer>,
) {
    let radius = 0.2;

    let mut rng = rand::thread_rng();
    let half_size = world_settings.size / 2.1;
    for _ in 0..world_settings.fruits {
        let x = rng.gen_range(-half_size, half_size);
        let y = rng.gen_range(-half_size, half_size);

        commands
            .spawn_bundle((
                Transform::from_xyz(x, 10., y),
                GlobalTransform::identity(),
            ))
            // .insert_bundle(PointLightBundle {
            //     point_light: PointLight {
            //         intensity: 10.0,
            //         color: Color::ORANGE,
            //         shadows_enabled: false,
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // })
            .insert_bundle(RigidBodyBundle {
                position: RigidBodyPosition {
                    position: Isometry::new(
                        Vec3::new(x, 10., y).into(),
                        Vec3::new(0., 0., 0.).into(),
                    ),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                // shape: ColliderShape::ball(radius).into(),
                // shape: ColliderShape::cuboid(radius, radius, radius).into(),
                shape: ColliderShape::capsule(
                    Point3::new(0.0, -0.1, 0.0),
                    Point3::new(0.0, 0.1, 0.0),
                    radius,
                )
                .into(),
                // flags: ColliderFlags {
                //     collision_groups: InteractionGroups::all(),
                //     ..ColliderFlags::default()
                // }
                // .into(),
                material: ColliderMaterial::new(1., 0.1).into(),
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            .with_children(|parent| {
                parent.spawn_scene(ass.load("models/fruits/tangerine.glb#Scene0"));
            });
    }

}
