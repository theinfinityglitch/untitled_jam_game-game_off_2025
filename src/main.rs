use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(960, 540),
                        title: String::from("Untitled Jam Game"),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(960.0 / 4.0, 540.0 / 4.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test_ldtk.ldtk").into(),
        ..Default::default()
    });
}
