mod game_flow;
mod player;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(960, 540),
                        title: String::from("Jam Game"),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LdtkPlugin)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()))
        .add_plugins(game_flow::GameFlowPlugin)
        .insert_resource(LevelSelection::index(0))
        .run();
}
