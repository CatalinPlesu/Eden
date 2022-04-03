use bevy::prelude::shape;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_rapier3d::prelude::*;
use rand::Rng;

pub fn spawn_gltf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    // let tree0 = ass.load("tree0.glb#Scene0");
    // let tree1 = ass.load("tree1.glb#Scene0");
    // let bush0 = ass.load("bush0.glb#Scene0");
    // let bush1 = ass.load("bush1.glb#Scene0");

    // for i in -20..=20 {
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::rgb(1. + i as f32 / 10., 0.5, 1. + i as f32 / 20.),
    //             unlit: true,
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     })
    //     .insert(Transform::from_xyz(0., i as f32, 0.));
    // }

    // for x in (-100..=100).step_by(10) {
    //     for z in (-100..=100).step_by(10) {
    //         commands
    //             .spawn_bundle((
    //                 Transform::from_xyz(x as f32, -10.0, z as f32),
    //                 GlobalTransform::identity(),
    //             ))
    //             .with_children(|parent| {
    //                 parent.spawn_scene(ass.load("tree0.glb#Scene0"));
    //             });
    //     }
    // }

    for mesh in meshes.iter() {
        let mesh = &mesh;

        let vertices: Vec<Vec3> = match mesh.1.attribute(Mesh::ATTRIBUTE_POSITION) {
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
        let mut vals: Vec<usize> = (0..500).map(|_| rng.gen_range(0, vertices.len())).collect();
        vals.sort();
        vals.dedup();
        println!("{:?}", vertices);
        println!("{}", vals.len());
        println!("{}", vertices.len());

        for i in vals {
            let p: f64 = rng.gen();
            commands
                .spawn_bundle((
                    Transform::from_xyz(
                        vertices[i].x as f32 -100.,
                        vertices[i].y as f32 - 10.,
                        vertices[i].z as f32 -100.,
                        ),
                    GlobalTransform::identity(),
                ))
                .with_children(|parent| {
                    if p > 0.80 {
                        parent.spawn_scene(ass.load("bush0.glb#Scene0"));
                    } else if p > 0.60 {
                        parent.spawn_scene(ass.load("bush1.glb#Scene0"));
                    } else if p > 0.40 {
                        parent.spawn_scene(ass.load("tree0.glb#Scene0"));
                    } else {
                        parent.spawn_scene(ass.load("tree1.glb#Scene0"));
                    }
                });
        }
        break;
    }

    // commands
    //     .spawn_bundle((
    //         Transform::from_xyz(-6.0, -10.0, 0.0),
    //         GlobalTransform::identity(),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn_scene(tree1);
    //     });

    // commands
    //     .spawn_bundle((
    //         Transform::from_xyz(0.0, -10.0, -5.0),
    //         GlobalTransform::identity(),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn_scene(bush0);
    //     });

    // commands
    //     .spawn_bundle((
    //         Transform::from_xyz(3.0, -10.0, 0.0),
    //         GlobalTransform::identity(),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn_scene(bush1);
    //     });
}
