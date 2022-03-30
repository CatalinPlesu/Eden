use std::f32::consts::TAU;
use std::slice::Iter;

use bevy::{
    app::Plugin,
    core::{cast_slice, Pod},
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
    },
    window::WindowDescriptor,
};
use bevy_rapier3d::{
    na::Point3,
    prelude::*,
};

pub use input::*;
pub use controller::*;

mod input;
mod controller;

pub struct BufVec<T: Pod> {
    values: Vec<T>,
    buffer: Option<Buffer>,
    capacity: usize,
    item_size: usize,
    buffer_usage: BufferUsages,
}

impl<T: Pod> Default for BufVec<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            buffer: None,
            capacity: 0,
            buffer_usage: BufferUsages::all(),
            item_size: std::mem::size_of::<T>(),
        }
    }
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
        .add_system_to_stage(CoreStage::PreUpdate, player_input_system)
        .add_system(cursor_grab_system)
        .add_system_set(SystemSet::new()
            .with_system(player_look_system)
            .with_system(player_move_system)
        )
        .add_system_to_stage(CoreStage::PostUpdate, sync_player_camera_system)
        ;
    }
}



pub fn setup_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.insert_resource(Config::default());

    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());

    commands.spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::capsule(Point3::new(0.0, 0.5, 0.0), Point3::new(0.0, 1.5, 0.0), 0.5).into(),
            collider_type: ColliderType::Solid.into(),
            ..Default::default()
        })
        .insert(ColliderDebugRender::with_id(0))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: Vec3::new(4.0, 24.0, 4.0).into(),
            activation: RigidBodyActivation::cannot_sleep().into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default() }.into(),
            ..Default::default()
        })
        .insert(PlayerInput {
            pitch: -TAU / 12.0,
            yaw: TAU * 5.0 / 8.0,
            ..Default::default()
        })
        .insert(PlayerController {
            ..Default::default()
        });

}
