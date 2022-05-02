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

pub struct WhalePlugin;

#[derive(Debug, Component)]
pub struct Whale {
    pub timer: Timer,
    pub point: [Vec3; 4],
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
        let distance = distance(&points[0], &points[3]);
        let velocity = distance / 20.;

        Self {
            timer: Timer::from_seconds(velocity, false),
            point: points,
            bounds: bounds,
        }
    }
}

pub fn distance(a: &Vec3, b: &Vec3) -> f32 {
    ((b.x - a.x).powf(2.) + (b.y - a.y).powf(2.) + (b.z - a.z).powf(2.)).sqrt()
}

pub fn gen_point(half_size: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-half_size, half_size);
    let z = rng.gen_range(-half_size, half_size);
    let y = rng.gen_range(20., f32::max(100., 2. * half_size));
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
        app.add_startup_system(setup).add_system(whale_move_system);
    }
}

fn whale_move_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Whale)>,
) {
    for (mut transform, mut whale) in query.iter_mut() {
        if !whale.timer.finished() {
            whale.timer.tick(time.delta());
            let t =
                (whale.timer.elapsed().as_secs_f64() / whale.timer.duration().as_secs_f64()) as f32;

            let p = quadratic_bezier_curve(&t, &whale.point);

            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.45,
                    subdivisions: 32,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.2, 0.2, 0.8),
                    metallic: 0.5,
                    perceptual_roughness: 0.5,
                    ..Default::default()
                }),
                transform: Transform::from_translation(p),
                ..Default::default()
            });

            transform.translation = p;

            let direction = (quadratic_bezier_curve(&(t + 0.1), &whale.point) - p).normalize();
            let yaw = f32::atan2(direction.x, direction.z);
            let pitch = f32::asin(direction.y);
            transform.rotation =
                Quat::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::PI + yaw, pitch);
        } else {
            let new_whale = Whale::new(whale.bounds, whale.point[3]);
            whale.timer = new_whale.timer;
            whale.point = new_whale.point;
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
    let half_size = world_settings.size;
    let mut rng = rand::thread_rng();

    for _ in 0..world_settings.whales {
        // let start = gen_point(half_size);
        let start = Vec3::new(0., 10., 0.);
        let scale = rng.gen_range(5., 10.);
        let whale = Whale::new(half_size, start);

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

        let direction = (whale.point[3] - whale.point[0]).normalize();
        let yaw = f32::atan2(direction.x, direction.z);
        let pitch = f32::asin(direction.y);
        println!("{:?}", (yaw, pitch));

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
