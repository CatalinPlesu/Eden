use bevy::prelude::*;

pub struct PrototypeGamePlugin;

impl Plugin for PrototypeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(print_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn().insert_bundle(PlayerBundle {
        name: Name("Catalin".to_string()),
        ..Default::default()
    });
}

fn print_player(query: Query<&Name ,With<Player>>) {
    for name in query.iter() {
        println!("player {}", name.0);
    }
}

#[derive(Default, Component)]
struct NPC;

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component)]
struct Name(String);

#[derive(Default, Component)]
struct Vitals {
    health: f32,
    stamina: f32,
}

#[derive(Default, Component)]
struct Attributes {
    strength: f32,
}

#[derive(Default, Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Default, Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    name: Name,
    vitals: Vitals,
    attributes: Attributes,
    position: Position,
    velocity: Velocity,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PrototypeGamePlugin)
        .run();
}

