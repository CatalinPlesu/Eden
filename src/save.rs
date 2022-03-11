use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
};

// extern crate rapier3d as rapier; // For the debug UI.
// use bevy_rapier3d::prelude::*;
// use rapier::geometry::ColliderShape;
// use rapier3d::pipeline::PhysicsPipeline;
// use ui::DebugUiPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(LightPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(CubePlugin)
        .run();
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_camera);
    }
}

fn generate_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub struct CubePlugin;
impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_cube);
    }
}

fn generate_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.8, 0.0),
        ..Default::default()
    })
    .insert(Wireframe);
}

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_light);
    }
}

fn generate_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_plane);
    }
}

fn generate_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -1..=1 {
        for z in -1..=1 {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_xyz(x as f32, 0., z as f32),
                ..Default::default()
            })
            .insert(Wireframe);
        }
    }
}

///#############################
use bevy::prelude::*;

pub struct PrototypeGamePlugin;

impl Plugin for PrototypeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(print_vitals)
            .add_system(print_player_name);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn().insert_bundle(PlayerBundle {
        name: Name("Catalin".to_string()),
        ..Default::default()
    });
}

fn print_player_name(query: Query<&Name, With<Player>>) {
    for name in query.iter() {
        println!("player {}", name.0);
    }
}

fn print_vitals(query: Query<&Vitals>) {
    for vital in query.iter() {
        println!("Vitals: \n{:?}", vital);
    }
}

#[derive(Default, Component)]
struct NPC;

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component)]
struct Name(String);

#[derive(Debug, Component)]
struct Vitals {
    health: f32,
    stamina: f32,
}

impl Default for Vitals {
    fn default() -> Self {
        Self {
            health: 100.,
            stamina: 100.,
        }
    }
}

#[derive(Component)]
struct Attributes {
    strength: f32,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            strength: 20.,
        }
    }
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


/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}

