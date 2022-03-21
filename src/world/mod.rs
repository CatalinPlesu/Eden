use bevy::{
    app::Plugin,
    prelude::*,
};
pub mod terrain;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(terrain::generate_terrain);
    }
}
