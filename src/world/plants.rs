use crate::world::MyRaycastSet;
use bevy::prelude::shape;
use bevy::prelude::*;
use bevy_mod_raycast::*;
use std::f32::consts::FRAC_PI_2;

pub fn spawn_gltf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
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

    for x in (-100..=100).step_by(10) {
        for z in (-100..=100).step_by(10) {
            commands
                .spawn_bundle((
                    Transform::from_xyz(x as f32, -10.0, z as f32),
                    GlobalTransform::identity(),
                ))
                .with_children(|parent| {
                    parent.spawn_scene(ass.load("tree0.glb#Scene0"));
                });
        }
    }

    // for x in [-1., 0., 1.] {
    //     for y in [-1., 0., 1.] {
    //         for z in [-1., 0., 1.] {
    //             let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    //             let ray = Ray::new(
    //                 Vec3::new(0., 30.0, 0.).into(),
    //                 Vec3::new(x, y, z).into(),
    //             );
    //             let max_toi = 6000.0;
    //             let solid = true;
    //             let groups = InteractionGroups::all();
    //             let filter = None;

    //             if let Some((handle, toi)) =
    //                 query_pipeline.cast_ray(&collider_set, &ray, max_toi, solid, groups, filter)
    //             {
    //                 // The first collider hit has the handle `handle` and it hit after
    //                 // the ray travelled a distance equal to `ray.dir * toi`.
    //                 let hit_point = ray.point_at(toi); // Same as: `ray.origin + ray.dir * toi`
    //                 println!("Entity {:?} hit at point {}, dir {:?}", handle.entity(), hit_point, (x, y, z));
    //             }
    //         }
    //     }
    // }
    // commands
    //     .spawn_bundle((
    //         Transform::from_xyz(0.0, -10.0, 0.0),
    //         GlobalTransform::identity(),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn_scene(tree0);
    //     });

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

pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<MyRaycastSet>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}
