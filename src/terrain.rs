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
use bevy::prelude::*; 
use bevy::render::mesh::*; 
use bevy::render::render_resource::PrimitiveTopology;
use noise::{utils::PlaneMapBuilder, Perlin, Seedable, utils::NoiseMapBuilder};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_half = 50.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(create_mesh(400)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0., 0.5, 0.3).into(),
                metallic: 1.,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(-100.0, 0.0, -100.0),
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

fn create_mesh(n: u32) -> Mesh {  
    let perlin = Perlin::new()
    .set_seed(201);
    let noise_map = PlaneMapBuilder::new(&perlin)
    .set_size(n as usize, n as usize)
    .set_x_bounds(-5.0, 5.0)
    .set_y_bounds(-5.0, 5.0)
    .build();

    // let indices = Indices::U32(vec![0, 2, 1]);
    let mut indices: Vec<u32> =  vec!{};
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    
    // generate vertices
    for x in 0..n+1 {
        for z in 0..n+1 {
            positions.push([x as f32, noise_map.get_value(x as usize, z as usize) as f32 * 10.0, z as f32]);
            normals.push([0., 1.0, 0.]);
            uvs.push([1.,1.]);
        }
    }

    // generate indicies 
    for x in 0..n {
        for z in 0..n {
            // create lower triange
            let a = (n*x)+x+z;
            let b = a+1;
            let c = (x+1)*n+x+1+z;
            let d = c+1; 

            // Lower triangle
            indices.push(a);
            indices.push(b);
            indices.push(c);

            // Upper triangle
            indices.push(d);
            indices.push(c);
            indices.push(b);
            
            
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
