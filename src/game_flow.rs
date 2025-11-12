use crate::player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut rapier_config: Query<&mut RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<(), BevyError> {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(640.0 / 2.0, 360.0 / 2.0, 0.0),
    ));

    // Spawn a blue square for representating a test player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0f32, 25.0f32))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.0f32, 0.0f32, 1.0f32))),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(25.0f32 / 2.0f32, 25.0f32 / 2.0f32),
        KinematicCharacterController::default(),
        Transform::from_xyz(640.0f32 / 2.0f32, 360.0f32 / 2.0f32, 3.0f32),
        player::Player,
    ));

    // rapier_config.single_mut()?.gravity = Vec2::new(0.0, -2000.0);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test_ldtk.ldtk").into(),
        ..Default::default()
    });

    Ok(())
}

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player::player_system)
            .add_systems(Update, player::jump)
            .add_systems(Update, player::rise)
            .add_systems(Update, player::fall);
    }
}
