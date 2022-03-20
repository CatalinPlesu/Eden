use bevy::{
    app::Plugin,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
};
use bevy_rapier3d::{
    physics::wrapper::{
        ColliderMassPropsComponent, ColliderMaterialComponent, RigidBodyMassPropsComponent,
    },
    prelude::*,
    render::RapierRenderPlugin,
};
use rapier3d::prelude::ColliderBuilder;
// use rapier3d::na::{DMatrix, Matrix, Vector3};
use crate::terrain::nalgebra::Vector3;
use bevy::render::mesh::*;
use bevy::render::render_resource::PrimitiveTopology;
use noise::{utils::NoiseMapBuilder, utils::PlaneMapBuilder, Perlin, Seedable};
use rapier3d::na::ComplexField;

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
    let mesh_collider = create_mesh(100);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh_collider.0),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0., 0.5, 0.3).into(),
                metallic: 1.,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Wireframe)
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(mesh_collider.1),
            // shape: ColliderShape::cuboid(200., 0., 200.).into(),
            ..Default::default()
        });
}

fn create_mesh(n: u32) -> (Mesh, ColliderShape) {
    let perlin = Perlin::new().set_seed(201);
    let noise_map = PlaneMapBuilder::new(&perlin)
        .set_size(n as usize, n as usize)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .build();

    // let indices = Indices::U32(vec![0, 2, 1]);
    let mut indices: Vec<u32> = vec![];
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    // let mut heights = Vec::new();

    // generate vertices
    for x in 0..n + 1 {
        // let mut height = Vec::new();
        for z in 0..n + 1 {
            let y = noise_map.get_value(x as usize, z as usize) as f32 * 10.0;
            positions.push([x as f32, y, z as f32]);
            // height.push(y);
            normals.push([0., 1.0, 0.]);
            uvs.push([1., 1.]);
        }
        // heights.push(height);
    }

    let ground_size: Vector3<Real> = Vector3::<Real>::new(100.0, 1.0, 100.0);
    let nsubdivs = 20;

    let heights = DMatrix::from_fn(nsubdivs + 1, nsubdivs + 1, |i, j| {
        if i == 0 || i == nsubdivs || j == 0 || j == nsubdivs {
            10.0
        } else {
            let x = i as f32 * ground_size.x / (nsubdivs as f32);
            let z = j as f32 * ground_size.z / (nsubdivs as f32);
            ComplexField::sin(x) + ComplexField::cos(z)
        }
    });

    // let collider = ColliderBuilder::heightfield(heights, ground_size).build();
    let collider_shape = ColliderShape::heightfield(heights, ground_size);

    // generate indicies
    for x in 0..n {
        for z in 0..n {
            // create lower triange
            let a = (n * x) + x + z;
            let b = a + 1;
            let c = (x + 1) * n + x + 1 + z;
            let d = c + 1;

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
    (mesh, collider_shape)
}
