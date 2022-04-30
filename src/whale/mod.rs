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
pub struct Whale;

impl Plugin for WhalePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(whale_move_system);
    }
}


fn whale_move_system(mut query: Query<&mut Transform, With<Whale>>) {
    for mut transform in query.iter_mut() {
        // println!("{:?}", transform);
        transform.translation.z -= 0.1;
        transform.translation.y += 0.01;
    }
}


fn setup(
    mut commands: Commands,
    world_settings: Res<WorldSettings>,
    asset_server: Res<AssetServer>,
) {
    let half_size = world_settings.size / 2.;
    let mut rng = rand::thread_rng();

    for _ in 0..world_settings.whales {

        let x = rng.gen_range(-half_size, half_size);
        let z = rng.gen_range(-half_size, half_size);
        let y = rng.gen_range(10., 30.);
        let scale = rng.gen_range(5., 10.);

        commands
            .spawn_bundle((
                Transform::from_xyz(x, y, z).with_scale(Vec3::new(scale, scale, scale)),
                GlobalTransform::identity(),
            ))
            .insert(Whale)
            .with_children(|parent| {
                parent.spawn_scene(asset_server.load("whale.glb#Scene0"));
            });
    }
}
