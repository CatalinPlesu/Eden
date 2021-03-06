use bevy::{app::Plugin, prelude::*};
// pub mod plants;
pub mod sun;
pub mod terrain;
use terrain::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("87ceeb").unwrap()));
        app.insert_resource(AmbientLight {
            color: Color::rgb(0.2, 0.5, 0.5),
            brightness: 1.0,
        });

        app.add_startup_system(sun::setup);

        app.add_startup_system(terrain::generate_terrain);
        // app.add_startup_system(plants::spawn_gltf);

        // app.add_startup_system(terrain::generate_terrain.system().label("load"))
        // .add_startup_system(plants::spawn_gltf.system().after("load"));
    }
}
