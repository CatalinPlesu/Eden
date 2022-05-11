use crate::*;
use crate::*;
use bevy::text::Font;
use bevy::{app::Plugin, prelude::*};
use bevy::{
    diagnostic::Diagnostics,
    diagnostic::FrameTimeDiagnosticsPlugin,
    math::Rect,
    prelude::{AssetServer, Color, Component, QuerySet, QueryState, TextBundle},
    prelude::{Query, With},
    text::{TextSection, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};
use bevy::{ecs::system::Res, prelude::Commands};
use bevy::{prelude::Handle, text::Text};
use bevy_rapier3d::prelude::{
    MassProperties, RigidBodyMassPropsComponent, RigidBodyVelocityComponent,
};
use rand::prelude::*;

const DEBUG: bool = false;
const TANGERINE: bool = true;

pub struct WhalePlugin;

#[derive(Debug, Component)]
struct TrailTime {
    timer: Timer,
}

#[derive(Debug, Component)]
pub struct Whale {
    pub timer: Timer,
    pub spawn_timer: Timer,
    pub point: [Vec3; 4],
    pub color: Color,
    pub velocity: f32,
    pub bounds: f32,
}

impl Whale {
    pub fn new(bounds: f32, start: Vec3) -> Self {
        let points = [
            start,
            gen_point(bounds),
            gen_point(bounds),
            gen_point(bounds),
        ];
        let mut rng = rand::thread_rng();

        let velocity = rng.gen_range(4., 8.);
        let time = distance(&points[0], &points[3]) / velocity;

        Self {
            timer: Timer::from_seconds(time, false),
            spawn_timer: Timer::from_seconds(0.5, true),
            point: points,
            color: Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
            velocity,
            bounds,
        }
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        let t = rng.gen_range(-1., 0.);
        let points = [
            self.point[3],
            Vec3::new(
                (self.point[2].x - self.point[3].x) * t + self.point[3].x,
                (self.point[2].y - self.point[3].y) * t + self.point[3].y,
                (self.point[2].z - self.point[3].z) * t + self.point[3].z,
            ),
            gen_point(self.bounds),
            gen_point(self.bounds),
        ];
        let time = distance(&points[0], &points[3]) / self.velocity;

        self.timer = Timer::from_seconds(time, false);
        self.point = points;
        self.color = Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
    }
}

pub fn distance(a: &Vec3, b: &Vec3) -> f32 {
    ((b.x - a.x).powf(2.) + (b.y - a.y).powf(2.) + (b.z - a.z).powf(2.)).sqrt()
}

pub fn gen_point(half_size: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-half_size, half_size);
    let z = rng.gen_range(-half_size, half_size);
    let y = rng.gen_range(20., 100.);
    Vec3::new(x, y, z)
}

pub fn quadratic_bezier_curve(t: &f32, p: &[Vec3; 4]) -> Vec3 {
    let x = (1. - t).powf(3.) * p[0].x
        + 3. * (1. - t).powf(2.) * t * p[1].x
        + 3. * (1. - t) * t.powf(2.) * p[2].x
        + t.powf(3.) * p[3].x;
    let y = (1. - t).powf(3.) * p[0].y
        + 3. * (1. - t).powf(2.) * t * p[1].y
        + 3. * (1. - t) * t.powf(2.) * p[2].y
        + t.powf(3.) * p[3].y;
    let z = (1. - t).powf(3.) * p[0].z
        + 3. * (1. - t).powf(2.) * t * p[1].z
        + 3. * (1. - t) * t.powf(2.) * p[2].z
        + t.powf(3.) * p[3].z;
    Vec3::new(x, y, z)
}

impl Plugin for WhalePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(whale_move_system)
            .add_system(trail_despawn);
    }
}

fn whale_move_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Whale)>,
    world_settings: Res<WorldSettings>,
) {
    for (mut transform, mut whale) in query.iter_mut() {
        if !whale.timer.finished() {
            whale.timer.tick(time.delta());
            whale.spawn_timer.tick(time.delta());
            let t =
                (whale.timer.elapsed().as_secs_f64() / whale.timer.duration().as_secs_f64()) as f32;

            let p = quadratic_bezier_curve(&t, &whale.point);

            if whale.spawn_timer.finished() {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: 0.45,
                            subdivisions: 32,
                        })),
                        material: materials.add(StandardMaterial {
                            // base_color: Color::hex("ff3403").unwrap(),
                            base_color: whale.color,
                            metallic: 0.5,
                            perceptual_roughness: 0.5,
                            ..Default::default()
                        }),
                        transform: Transform::from_translation(p),
                        ..Default::default()
                    })
                    .insert(TrailTime {
                        timer: Timer::from_seconds(75. / world_settings.whales as f32, false),
                    });
            }

            transform.translation = p;

            let direction = (quadratic_bezier_curve(&(t + 0.1), &whale.point) - p).normalize();
            let yaw = f32::atan2(direction.x, direction.z);
            let pitch = f32::asin(direction.y);
            transform.rotation =
                Quat::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::PI + yaw, pitch);
        } else {
            // let new_whale = Whale::new(whale.bounds, whale.point[3]);
            // whale.timer = new_whale.timer;
            // whale.point = new_whale.point;
            whale.reset();
            if DEBUG {
                let color = [
                    Color::hex("355C7D").unwrap(),
                    Color::hex("A8A7A7").unwrap(),
                    Color::hex("E1F5C4").unwrap(),
                    Color::hex("F67280").unwrap(),
                ];
                for (i, vec3) in whale.point.iter().enumerate() {
                    commands.spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: 5.,
                            subdivisions: 32,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: color[i],
                            metallic: 0.5,
                            perceptual_roughness: 0.5,
                            ..Default::default()
                        }),
                        transform: Transform::from_translation(*vec3),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    world_settings: Res<WorldSettings>,
    asset_server: Res<AssetServer>,
) {
    let half_size = world_settings.size / 2.;
    let mut rng = rand::thread_rng();

    for _ in 0..world_settings.whales {
        let start = gen_point(half_size);
        let scale = rng.gen_range(5., 10.);
        let whale = Whale::new(half_size, start);

        if DEBUG {
            let color = [
                Color::hex("355C7D").unwrap(),
                Color::hex("A8A7A7").unwrap(),
                Color::hex("E1F5C4").unwrap(),
                Color::hex("F67280").unwrap(),
            ];
            for (i, vec3) in whale.point.iter().enumerate() {
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 5.,
                        subdivisions: 32,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: color[i],
                        metallic: 0.5,
                        perceptual_roughness: 0.5,
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(*vec3),
                    ..Default::default()
                });
            }
        }

        let direction = (whale.point[3] - whale.point[0]).normalize();
        let yaw = f32::atan2(direction.x, direction.z);
        let pitch = f32::asin(direction.y);

        commands
            .spawn_bundle((
                Transform::from_xyz(start.x, start.y, start.z)
                    .with_scale(Vec3::new(scale, scale, scale))
                    .with_rotation(Quat::from_euler(
                        EulerRot::ZYX,
                        0.0,
                        std::f32::consts::PI + yaw,
                        pitch,
                    )),
                GlobalTransform::identity(),
            ))
            .insert(whale)
            .with_children(|parent| {
                parent.spawn_scene(asset_server.load("whale.glb#Scene0"));
            });
    }
}

fn trail_despawn(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>,
    ass: Res<AssetServer>,
    mut commands: Commands,
    mut q: Query<(Entity, &Transform, &mut TrailTime)>,
    time: Res<Time>,
) {
    for (entity, transform, mut trail_timer) in q.iter_mut() {
        trail_timer.timer.tick(time.delta());

        // if it finished, despawn the bomb
        if trail_timer.timer.finished() {
            commands.entity(entity).despawn();

            if world_settings.fruits < world_settings.fruits_limit {
                let mut rng = rand::thread_rng();
                let p: f32 = rng.gen::<f32>();
                let scale = 5.;
                let radius = 0.12 * scale;
                if p > 0.7 && TANGERINE {
                    commands
                        .spawn_bundle((
                            Transform::from_xyz(
                                transform.translation.x,
                                transform.translation.y,
                                transform.translation.z,
                            )
                            .with_scale(Vec3::new(scale, scale, scale)),
                            GlobalTransform::identity(),
                        ))
                        .insert(Tangerine)
                        .insert_bundle(RigidBodyBundle {
                            position: RigidBodyPosition {
                                position: Isometry::new(
                                    Vec3::new(
                                        transform.translation.x,
                                        transform.translation.y,
                                        transform.translation.z,
                                    )
                                    .into(),
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
        }
    }
}
