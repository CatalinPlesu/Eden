use bevy::prelude::shape;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::world::terrain::Terrain;

pub fn spawn_gltf(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
    terrain: Res<Terrain>,
) {
    // for mesh in meshes.iter() {
    //     let mesh = &mesh;

    //     let vertices: Vec<Vec3> = match mesh.1.attribute(Mesh::ATTRIBUTE_POSITION) {
    //         None => panic!("Mesh does not contain vertex positions"),
    //         Some(vertex_values) => match &vertex_values {
    //             VertexAttributeValues::Float32x3(positions) => positions
    //                 .iter()
    //                 .map(|coordinates| Vec3::from(*coordinates))
    //                 .collect(),
    //             _ => panic!("Unexpected vertex types in ATTRIBUTE_POSITION"),
    //         },
    //     };

    //     let mut rng = rand::thread_rng();
    //     let mut vals: Vec<usize> = (0..500).map(|_| rng.gen_range(0, vertices.len())).collect();
    //     vals.sort();
    //     vals.dedup();
    //     println!("{:?}", vertices);
    //     println!("{}", vals.len());
    //     println!("{}", vertices.len());

    //     for i in vals {
    //         let p: f64 = rng.gen();
    //         commands
    //             .spawn_bundle((
    //                 Transform::from_xyz(
    //                     vertices[i].x as f32 -100.,
    //                     vertices[i].y as f32 - 10.,
    //                     vertices[i].z as f32 -100.,
    //                     ),
    //                 GlobalTransform::identity(),
    //             ))
    //             .with_children(|parent| {
    //                 if p > 0.80 {
    //                     parent.spawn_scene(ass.load("bush0.glb#Scene0"));
    //                 } else if p > 0.60 {
    //                     parent.spawn_scene(ass.load("bush1.glb#Scene0"));
    //                 } else if p > 0.40 {
    //                     parent.spawn_scene(ass.load("tree0.glb#Scene0"));
    //                 } else {
    //                     parent.spawn_scene(ass.load("tree1.glb#Scene0"));
    //                 }
    //             });
    //     }
    //     break;
    // }

}
