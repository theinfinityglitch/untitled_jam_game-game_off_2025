use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(960, 540),
                title: String::from("Untitled Jam Game"),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .run();
}
