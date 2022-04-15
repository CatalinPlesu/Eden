use std::f32::consts::TAU;

use bevy::{
    app::Plugin,
    prelude::*,
};
use bevy_rapier3d::{
    na::Point3,
    prelude::*,
};

pub use input::*;
pub use controller::*;
pub use structures::*;

mod input;
mod controller;
mod structures;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
        .add_system_to_stage(CoreStage::PreUpdate, player_input_system)
        .add_system(cursor_grab_system)
        .add_system(player_move_system)
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

    commands.insert_resource(InputMap::default());

    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());

    commands.spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::capsule(Point3::new(0.0, 0., 0.0), Point3::new(0.0, 1.0, 0.0), 0.5).into(),
            collider_type: ColliderType::Solid.into(),
            ..Default::default()
        })
        .insert(ColliderDebugRender::with_id(0))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: Vec3::new(0., 10., 0.).into(),
            activation: RigidBodyActivation::cannot_sleep().into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default() }.into(),
            ..Default::default()
        })
        .insert(PlayerController {
            ..Default::default()
        }
        )
        ;

}
