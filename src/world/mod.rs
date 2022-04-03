use bevy::{app::Plugin, prelude::*};
use bevy_mod_raycast::*;
pub mod plants;
pub mod sun;
pub mod terrain;

pub struct MyRaycastSet;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("87ceeb").unwrap()));
        app.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
        app.insert_resource(RayCastSource::<MyRaycastSet>::new_transform_empty());
        app.insert_resource(AmbientLight {
            color: Color::rgb(0.2, 0.5, 0.5),
            brightness: 0.5,
        });

        app.add_startup_system(sun::setup);

        app.add_startup_system(terrain::generate_terrain);

        app.add_startup_system(plants::spawn_gltf);

    }
}
