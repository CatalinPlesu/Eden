use crate::*;
use bevy::text::Font;
use bevy::{app::Plugin, prelude::*};
use bevy::{
    diagnostic::Diagnostics,
    diagnostic::FrameTimeDiagnosticsPlugin,
    math::Rect,
    prelude::{AssetServer, Color, Component, QuerySet, QueryState, TextBundle},
    prelude::{Query, With},
    text::{TextSection, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};
use bevy::{ecs::system::Res, prelude::Commands};
use bevy::{prelude::Handle, text::Text};
use bevy_rapier3d::prelude::{
    MassProperties, RigidBodyMassPropsComponent, RigidBodyVelocityComponent,
};

pub struct UserInterfacePlugin;
impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(fps_update_system)
            .add_system(score_update_system)
            .init_resource::<Countdown>()
            .add_system(time_update_system)
            .add_system(text_color_system);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(25.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Time: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TimeText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(45.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

pub struct Countdown {
    pub timer: Timer,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(60.0, false),
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new()
    }
}

fn time_update_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut query: Query<&mut Text, With<TimeText>>,
    mut game: ResMut<Game>,
) {
    countdown.timer.tick(time.delta());
    for mut text in query.iter_mut() {
        if !countdown.timer.finished() {
            text.sections[1].value = format!(
                "{}",
                (countdown.timer.duration() - countdown.timer.elapsed()).as_secs()
            );
        } else {
            text.sections[1].value = format!("{}", -1);
            game.state = GameState::Finished;

            commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(50.),
                            left: Val::Px(50.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "END!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(ColorText);
        }
    }
}

fn score_update_system(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", score.value);
    }
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        // We used the `Text::with_section` helper method, but it is still just a `Text`,
        // so to update it, we are still updating the one and only section
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}
