use bevy::{
    app::Plugin,
    prelude::*,
};
use bevy_rapier3d::{
    physics::wrapper::{
        ColliderMassPropsComponent, ColliderMaterialComponent, RigidBodyMassPropsComponent,
    },
    prelude::{
        ColliderBundle, ColliderMassProps, ColliderMaterial, ColliderPositionSync, ColliderShape,
        Isometry, NoUserData, Point, RapierPhysicsPlugin, Real, RigidBodyBundle, RigidBodyForces,
        RigidBodyMassProps, RigidBodyPosition, RigidBodyType, CoefficientCombineRule,
    },
    render::RapierRenderPlugin,
};

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
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0., 0.5, 0.3).into(),
                metallic: 1.,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(plane_half, 0., plane_half).into(),
            ..Default::default()
        });

}

