use bevy::{pbr::wireframe::*, prelude::*};
use bevy_rapier3d::prelude::*;
// use rapier3d::na::{DMatrix, Matrix, Vector3};
use crate::*;
use bevy::prelude::shape;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::mesh::*;
use bevy::render::render_resource::PrimitiveTopology;
use nalgebra::Vector3;
use noise::{utils::NoiseMapBuilder, utils::PlaneMapBuilder, Perlin, Seedable};
use rand::Rng;

pub fn generate_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
    world_settings: Res<WorldSettings>,
) {
    let size = world_settings.size.clone();

    let mesh_collider = create_mesh(size as u32, -1., 5.);

    let vertices: Vec<Vec3> = match mesh_collider.0.attribute(Mesh::ATTRIBUTE_POSITION) {
        None => panic!("Mesh does not contain vertex positions"),
        Some(vertex_values) => match &vertex_values {
            VertexAttributeValues::Float32x3(positions) => positions
                .iter()
                .map(|coordinates| Vec3::from(*coordinates))
                .collect(),
            _ => panic!("Unexpected vertex types in ATTRIBUTE_POSITION"),
        },
    };
    let mut rng = rand::thread_rng();
    let mut vals: Vec<usize> = (0..world_settings.plants)
        .map(|_| rng.gen_range(0, vertices.len()))
        .collect();
    vals.sort();
    vals.dedup();

    for i in vals {
        let p: f32 = rng.gen::<f32>() + world_settings.plant_dominance_offset;
        if world_settings.plants_colider {
            commands
                .spawn_bundle((
                    Transform::from_xyz(
                        vertices[i].x as f32 - size / 2.,
                        vertices[i].y as f32 - 0.1,
                        vertices[i].z as f32 - size / 2.,
                    ),
                    GlobalTransform::identity(),
                ))
                .insert_bundle(RigidBodyBundle {
                    position: RigidBodyPosition {
                        position: Isometry::new(
                            Vec3::new(
                                vertices[i].x as f32 - size / 2.,
                                vertices[i].y as f32 - 0.1,
                                vertices[i].z as f32 - size / 2.,
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
                    shape: ColliderShape::cuboid(0.3, 2., 0.3).into(),
                    // flags: ColliderFlags {
                    //     collision_groups: InteractionGroups::all(),
                    //     ..ColliderFlags::default()
                    // }
                    // .into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    if p > 0.75 {
                        parent.spawn_scene(ass.load("models/trees/bush0.glb#Scene0"));
                    } else if p > 0.50 {
                        parent.spawn_scene(ass.load("models/trees/bush1.glb#Scene0"));
                    } else if p > 0.25 {
                        parent.spawn_scene(ass.load("models/trees/tree0.glb#Scene0"));
                    } else {
                        parent.spawn_scene(ass.load("models/trees/tree1.glb#Scene0"));
                    }
                });
        } else {
            commands
                .spawn_bundle((
                    Transform::from_xyz(
                        vertices[i].x as f32 - size / 2.,
                        vertices[i].y as f32,
                        vertices[i].z as f32 - size / 2.,
                    ),
                    GlobalTransform::identity(),
                ))
                .with_children(|parent| {
                    if p > 0.75 {
                        parent.spawn_scene(ass.load("models/trees/bush0.glb#Scene0"));
                    } else if p > 0.50 {
                        parent.spawn_scene(ass.load("models/trees/bush1.glb#Scene0"));
                    } else if p > 0.25 {
                        parent.spawn_scene(ass.load("models/trees/tree0.glb#Scene0"));
                    } else {
                        parent.spawn_scene(ass.load("models/trees/tree1.glb#Scene0"));
                    }
                });
        }
    }

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh_collider.0),
            material: materials.add(StandardMaterial {
                // base_color: Color::rgb(0.9, 0.5, 0.5).into(),
                base_color: Color::rgb(0., 0.5, 0.3).into(),
                // base_color: Color::rgb(194., 178., 128.).into(),
                normal_map_texture: Some(asset_server.load("grass.jpg").clone()),
                // alpha_mode: AlphaMode::Blend,
                // unlit: true,
                metallic: 0.,
                perceptual_roughness: 1.,
                ..Default::default()
            }),
            transform: Transform::from_xyz(-size / 2., 0., -size / 2.),
            ..Default::default()
        })
        // .insert(Wireframe)
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: mesh_collider.1.into(),
            position: Isometry::new(Vec3::new(0., 0., 0.).into(), Vec3::new(0., 0., 0.).into())
                .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::all(),
                ..ColliderFlags::default()
            }
            .into(),
            ..Default::default()
        });

    ////no infinite fall
    //commands
    //    .spawn_bundle(ColliderBundle {
    //        shape: ColliderShape::cuboid(1000., 0.001, 1000.).into(),
    //        position: Isometry::new(
    //            Vec3::new(-500., -50., -500.).into(),
    //            Vec3::new(0., 0., 0.).into(),
    //        )
    //        .into(),
    //        ..Default::default()
    //    })
    //    .insert(ColliderDebugRender {
    //        color: Color::hex("f8b423").unwrap(),
    //    })
    //    .insert(ColliderPositionSync::Discrete);
}

fn create_mesh(n: u32, min: f64, max: f64) -> (Mesh, ColliderShape) {
    // let perlin = Perlin::new().set_seed(201);
    let perlin = Perlin::new();
    let noise_map = PlaneMapBuilder::new(&perlin)
        .set_size(n as usize, n as usize)
        .set_x_bounds(min, max)
        .set_y_bounds(min, max)
        .build();

    let mut indices: Vec<u32> = vec![];
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    let ground_size: Vector3<Real> = Vector3::<Real>::new((n + 1) as f32, 1., (n + 1) as f32);

    let mut heights = DMatrix::<Real>::from_element((n + 1) as usize, (n + 1) as usize, 0.);
    // generate vertices
    for x in 0..n + 1 {
        for z in 0..n + 1 {
            let y = noise_map.get_value(x as usize, z as usize) as f32 * 10.;
            positions.push([x as f32, y, z as f32]);
            normals.push([0., 1.0, 0.]);
            uvs.push([1., 1.]);
            heights[(z as usize, x as usize)] = y;
        }
    }

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
