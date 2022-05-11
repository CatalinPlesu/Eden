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
mod whale;

pub const RAPIER_PLAYER_GROUP: u32 = 1;
pub const RAPIER_TANGERINE_GROUP: u32 = 333;

pub struct WorldSettings {
    size: f32,
    plants: u32,
    plant_dominance_offset: f32,
    plants_colider: bool,
    fruits: i32,
    fruits_limit: i32,
    whales: u32,
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            size: 200.,
            plants: 100,
            plant_dominance_offset: 0.,
            plants_colider: true,
            fruits: 50,
            fruits_limit: 100,
            whales: 1,
        }
    }
}

pub struct Score {
    value: i32,
}
impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Finished,
}

pub struct Game {
    state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            state: GameState::Running,
        }
    }
}

#[derive(Debug, Component)]
pub struct Tangerine;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Eden"),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WorldSettings::default())
        .insert_resource(Score::default())
        .insert_resource(Game::default())
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
        .add_plugin(FruitsPlugin)
        // .add_plugin(ui::UserInterfacePlugin)
        .add_plugin(whale::WhalePlugin)
        .run();
}

pub struct FruitsPlugin;
impl Plugin for FruitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_fruits);
        app.add_system(tangerine_detection);
    }
}

fn spawn_fruits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>,
    ass: Res<AssetServer>,
) {
    let scale = 3.;
    let radius = 0.12 * scale;

    let mut rng = rand::thread_rng();
    let half_size = world_settings.size / 2.1;
    for _ in 0..world_settings.fruits {
        let x = rng.gen_range(-half_size, half_size);
        let y = rng.gen_range(-half_size, half_size);

        commands
            .spawn_bundle((
                Transform::from_xyz(x, 10., y).
                with_scale(Vec3::new(scale, scale, scale)),
                GlobalTransform::identity(),
            ))
            .insert(Tangerine)
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
                shape: ColliderShape::capsule(
                    Point3::new(0.0, -0.1, 0.0),
                    Point3::new(0.0, 0.1, 0.0),
                    radius,
                )
                .into(),
                material: ColliderMaterial::new(1., 0.1).into(),
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            .with_children(|parent| {
                parent.spawn_scene(ass.load("models/fruits/tangerine.glb#Scene0"));
            });
    }
}

pub fn tangerine_detection(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut camera_query: Query<&Transform, With<PerspectiveProjection>>,
    mut tangerines: Query<(Entity, &Transform, With<Tangerine>)>,
    mut world_settings: ResMut<WorldSettings>,
) {
    let radius = 2.2;
    let mut rng = rand::thread_rng();

    for transform in camera_query.iter_mut() {
        // println!("{:?}", transform.translation);
        // println!();
        for (tangerine, t_transform, wtf) in tangerines.iter_mut() {
            if (transform.translation.x - t_transform.translation.x).abs() < radius
                && (transform.translation.y - t_transform.translation.y).abs() < radius
                && (transform.translation.z - t_transform.translation.z).abs() < radius
            {
                // println!("{:?}", t_transform.translation);
                commands.entity(tangerine).despawn_recursive();
                score.value += rng.gen_range(0, 100);
                world_settings.fruits -= 1;
            }
            // commands.entity(tangerine).despawn_recursive();
        }
    }
}
